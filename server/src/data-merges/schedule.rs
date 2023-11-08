use std::collections::BTreeMap;
use anyhow::Context;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;
use crate::data_merges::MergeResult;
use crate::models;
use crate::models::schedule::{get_current_schedule_for_group_with_subject, ScheduleObjModel, WeekDay};

async fn get_new_link_id(con: &mut PoolConnection<Sqlite>) -> anyhow::Result<u32> {
    let res: Option<u32> = sqlx::query_scalar("SELECT MAX(link_id) as max FROM schedule_objs")
        .fetch_optional(&mut *con).await.context("Failed to fetch max link_id")?;

    Ok(res.unwrap_or(0) + 1)
}

async fn get_last_gen_id(con: &mut PoolConnection<Sqlite>, group_id: u32) -> anyhow::Result<u32> {
    let res: Option<u32> = sqlx::query_scalar("SELECT MAX(gen_id) as max FROM schedule_generation WHERE group_id = ?")
        .bind(group_id)
        .fetch_optional(&mut *con).await.context("Failed to fetch max gen_id")?;

    Ok(res.unwrap_or(0))
}

async fn create_new_gen(con: &mut PoolConnection<Sqlite>, gen_id: u32, group_id: u32) -> anyhow::Result<()> {
    sqlx::query("INSERT OR IGNORE INTO schedule_generation (gen_id, creation_time, group_id) VALUES (?, strftime('%s', 'now'), ?)")
        .bind(gen_id)
        .bind(group_id)
        .execute(&mut *con)
        .await.context("Failed to insert new schedule generation")?;

    Ok(())
}

/// group of schedule object with the same lesson
/// last gen id is used to reuse single new generation across merges
async fn single_schedule_obj_group_merge(group_id: u32, input_schedule_objs: &Vec<ScheduleObjModel>, subject_id: u32, last_gen_id: u32, con: &mut PoolConnection<Sqlite>) -> anyhow::Result<Vec<MergeResult>> {
    trace!("Merging single schedule object group");
    let mut input_schedule_objs = input_schedule_objs.clone();

    //check for unique position condition
    let mut unique_positions = BTreeMap::<(WeekDay, String, u32), u32>::new();
    for input_sched_obj in &input_schedule_objs {
        *unique_positions.entry((input_sched_obj.week_day, input_sched_obj.week_parity.clone(), input_sched_obj.time)).or_default() += 1;
    }

    for unique_position in unique_positions {
        if unique_position.1 > 1 {
            return Err(anyhow::anyhow!("Unique position condition in single_schedule_obj_group_merge has failed!"));
        }
    }

    let mut existing_sched_objs = get_current_schedule_for_group_with_subject(con, group_id, subject_id).await?;

    trace!("group_id: {}, subject_id: {}", group_id, subject_id);
    trace!("Found {} existing schedule objects", existing_sched_objs.len());
    trace!("Received {} input schedule objects", input_schedule_objs.len());


    let mut res = Vec::new();

    let latest_subject_gen = models::subjects::get_subjects_cur_gen(con).await?;
    let latest_teachers_gen = models::subjects::get_subjects_cur_gen(con).await?;

    //try to link by schedule_obj_id or get_lesson_pos
    for input_sched_obj in &input_schedule_objs {
        trace!("Searching link for input schedule object: {}", input_sched_obj.last_known_orig_sched_obj_id);
        let mut found = false;
        for existing_sched_obj in &mut existing_sched_objs {
            if input_sched_obj.get_lesson_pos() == existing_sched_obj.get_lesson_pos() {
                found = true;

                // process diff and update
                let mut diff = false;

                if input_sched_obj.teacher_id != existing_sched_obj.teacher_id
                    || input_sched_obj.second_teacher_id != existing_sched_obj.second_teacher_id
                    || input_sched_obj.third_teacher_id != existing_sched_obj.third_teacher_id
                    || input_sched_obj.fourth_teacher_id != existing_sched_obj.fourth_teacher_id
                    || input_sched_obj.auditorium != existing_sched_obj.auditorium
                    || input_sched_obj.get_lesson_pos() != existing_sched_obj.get_lesson_pos(){
                    diff = true;
                }


                if diff {
                    trace!("Detected diff in schedule object, updating...");

                    // invalidate old sched_obj (update gen_id)

                    let new_gen_id = last_gen_id + 1;
                    create_new_gen(con, new_gen_id, group_id).await?;

                    sqlx::query("UPDATE schedule_objs SET \
                        gen_end = ? \
                        WHERE schedule_obj_id = ? AND gen_end IS NULL")
                        .bind(new_gen_id)
                        .bind(existing_sched_obj.schedule_obj_id)
                        .execute(&mut *con)
                        .await.context("Failed to invalidate old schedule object")?;

                    // insert new object
                    sqlx::query("INSERT INTO schedule_objs \
            (last_known_orig_sched_obj_id, group_id, link_id, subject_id, subject_gen_id,\
            teacher_id, teacher_gen_id, second_teacher_id, third_teacher_id, fourth_teacher_id, auditorium,\
            updated_at, time, week_day, week_parity, gen_start, existence_diff) \
            VALUES\
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(input_sched_obj.last_known_orig_sched_obj_id)
                        .bind(group_id)
                        .bind(existing_sched_obj.link_id)
                        .bind(subject_id)
                        .bind(latest_subject_gen)
                        .bind(input_sched_obj.teacher_id)
                        .bind(latest_teachers_gen)
                        .bind(input_sched_obj.second_teacher_id)
                        .bind(input_sched_obj.third_teacher_id)
                        .bind(input_sched_obj.fourth_teacher_id)
                        .bind(input_sched_obj.auditorium.clone())
                        .bind(input_sched_obj.updated_at.clone())
                        .bind(input_sched_obj.time)
                        .bind(input_sched_obj.week_day)
                        .bind(input_sched_obj.week_parity.clone())
                        .bind(new_gen_id)
                        .bind("changed")
                        .execute(&mut *con)
                        .await.context("Failed to insert new schedule object")?;

                    info!("Schedule object [CHANGED]: ({}): {} week {}:{}", input_sched_obj.last_known_orig_sched_obj_id, input_sched_obj.week_parity,
                        input_sched_obj.week_day.to_string(), input_sched_obj.time);

                    res.push(MergeResult::Updated);
                }
                else {
                    trace!("No diff in schedule object, skipping...");
                    // btw update untracked information
                    if input_sched_obj.updated_at != existing_sched_obj.updated_at
                        || input_sched_obj.last_known_orig_sched_obj_id != existing_sched_obj.last_known_orig_sched_obj_id
                        || existing_sched_obj.subject_id != latest_subject_gen
                        || existing_sched_obj.teacher_id.map(|id| id != latest_teachers_gen).unwrap_or(false) {
                        info!("Updating untracked information for schedule object");

                        let new_teacher_gen_id = existing_sched_obj.teacher_id.map(|id| latest_teachers_gen);
                        sqlx::query("UPDATE schedule_objs SET \
                            updated_at = ?, \
                            last_known_orig_sched_obj_id = ?, \
                            subject_gen_id = ?, \
                            teacher_gen_id = ? \
                            WHERE schedule_obj_id = ?")
                            .bind(input_sched_obj.updated_at.clone())
                            .bind(input_sched_obj.last_known_orig_sched_obj_id)
                            .bind(latest_subject_gen)
                            .bind(new_teacher_gen_id)

                            .bind(existing_sched_obj.schedule_obj_id)
                            .execute(&mut *con)
                            .await.context("Failed to update schedule object")?;
                    }
                    res.push(MergeResult::NotModified);
                }

                break;
            }
        }
        if !found {
            trace!("Schedule object not found, inserting...");
            //process new schedule object
            let new_link_id = get_new_link_id(con).await?;
            let new_gen_id = last_gen_id + 1;

            create_new_gen(con, new_gen_id, group_id).await?;

            sqlx::query("INSERT INTO schedule_objs \
            (last_known_orig_sched_obj_id, group_id, link_id, subject_id, subject_gen_id,\
            teacher_id, teacher_gen_id, second_teacher_id, third_teacher_id, fourth_teacher_id, auditorium,\
            updated_at, time, week_day, week_parity, gen_start, existence_diff) \
            VALUES\
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(input_sched_obj.last_known_orig_sched_obj_id)
                .bind(group_id)
                .bind(new_link_id)
                .bind(subject_id)
                .bind(latest_subject_gen)
                .bind(input_sched_obj.teacher_id)
                .bind(latest_teachers_gen)
                .bind(input_sched_obj.second_teacher_id)
                .bind(input_sched_obj.third_teacher_id)
                .bind(input_sched_obj.fourth_teacher_id)
                .bind(input_sched_obj.auditorium.clone())
                .bind(input_sched_obj.updated_at.clone())
                .bind(input_sched_obj.time)
                .bind(input_sched_obj.week_day)
                .bind(input_sched_obj.week_parity.clone())
                .bind(new_gen_id)
                .bind("new")
                .execute(&mut *con)
                .await.context("Failed to insert new schedule object")?;
            info!("Schedule object [INSERTED]: ({}): {} week {}:{}", input_sched_obj.last_known_orig_sched_obj_id, input_sched_obj.week_parity,
                input_sched_obj.week_day.to_string(), input_sched_obj.time);

            res.push(MergeResult::Inserted);
        }
    }

    for existing_sched_obj in existing_sched_objs {
        let mut found = false;
        for input_sched_obj in &mut input_schedule_objs {
            if input_sched_obj.get_lesson_pos() == existing_sched_obj.get_lesson_pos() {
                found = true;
                break;
            }
        }
        if !found {
            // invalidate old sched_obj (update gen_id)
            trace!("Invalidating old schedule object: {}", existing_sched_obj.last_known_orig_sched_obj_id);

            let new_gen_id = last_gen_id + 1;
            create_new_gen(con, new_gen_id, group_id).await?;

            sqlx::query("UPDATE schedule_objs SET \
            gen_end = ? \
            WHERE schedule_obj_id = ? AND gen_end IS NULL")
                .bind(new_gen_id)
                .bind(existing_sched_obj.schedule_obj_id)
                .execute(&mut *con)
                .await.context("Failed to invalidate old schedule object")?;
        }
    }


    Ok(res)
}

pub async fn schedule_objs_merge(group_id: u32, schedule_objs: &Vec<ScheduleObjModel>, con: &mut PoolConnection<Sqlite>) -> anyhow::Result<()> {
    // group by subject_id

    let group_name = models::groups::get_group(con, group_id).await?.number;
    info!("MERGE::SCHEDULE_OBJ_GROUP Merging started! Group: ({}): {}", group_id, group_name);
    let start = std::time::Instant::now();
    let mut subj_id_to_sched_objs: std::collections::HashMap<u32, Vec<ScheduleObjModel>> = std::collections::HashMap::new();

    // group by subject id
    for sched_obj in schedule_objs {
        subj_id_to_sched_objs.entry(sched_obj.subject_id).or_default().push(sched_obj.clone());
    }

    let last_gen_id = get_last_gen_id(con, group_id).await?;
    info!("MERGE::SCHEDULE_OBJ_GROUP Last generation: {}", last_gen_id);

    let mut any_modified = false;
    let mut total_inserted_cnt = 0;
    let mut total_changed_cnt = 0;
    for (subj_id, subj_sched_objs) in subj_id_to_sched_objs {
        let mut modified = false;
        let mut inserted_cnt = 0;
        let mut changed_cnt = 0;

        trace!("MERGE::SCHEDULE_OBJ_GROUP Merging schedule objects for subject id {} started!", subj_id);
        let res = single_schedule_obj_group_merge(group_id, &subj_sched_objs, subj_id, last_gen_id, con).await?;
        for r in res {
            if r == MergeResult::Inserted {
                inserted_cnt += 1;
            }
            if r == MergeResult::Updated {
                changed_cnt += 1;
            }
            if r != MergeResult::NotModified {
                modified = true;
            }
        }

        total_inserted_cnt += inserted_cnt;
        total_changed_cnt += changed_cnt;

        if inserted_cnt > 0 || changed_cnt > 0 {
            info!("MERGE::SCHEDULE_OBJ_GROUP Schedule objects modified! Grouped by subject_id = {}", subj_id);
            info!("\tinserted: {}", inserted_cnt);
            info!("\tchanged: {}", changed_cnt);

            info!("MERGE::SCHEDULE_OBJ_GROUP Using generation id {}", last_gen_id + 1);
        }
    }

    models::groups::set_last_group_merge(group_id, con).await?;

    if any_modified {
        info!("MERGE::SCHEDULE_OBJ_GROUP Merge schedule objects for group finished with changes! New generation created with id {}", last_gen_id + 1);
        info!("\tinserted: {}", total_inserted_cnt);
        info!("\tchanged: {}", total_changed_cnt);
    }
    else {
        info!("MERGE::SCHEDULE_OBJ_GROUP Merge schedule objects: \tno changes!")
    }
    info!("MERGE::SCHEDULE_OBJ_GROUP Merging schedule objects for group finished in {:?}", start.elapsed());
    info!("");

    Ok(())
}
use crate::models::DbResult;
use anyhow::Context;
use rocket::time::PrimitiveDateTime;
use sqlx::PgConnection;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct SubjectModel {
    pub subject_obj_id: i32,
    pub subject_id: i32,

    // tracked with versioning
    pub title: String,
    pub short_title: String,
    pub subject_type: String,
    pub control_type: Option<String>,

    // untracked_info
    pub semester: i32,
    pub alien_id: i32,
    pub department_id: i32,

    // generation range info
    pub gen_start: i32,
    pub gen_end: Option<i32>,
    pub existence_diff: String,

    pub created_timestamp: PrimitiveDateTime,
    pub modified_timestamp: PrimitiveDateTime,
}

pub async fn get_subjects_cur_gen(con: &mut PgConnection) -> DbResult<i32> {
    let res: Option<i32> =
        sqlx::query_scalar!("SELECT MAX(gen_id) as max FROM subjects_generation")
            .fetch_one(&mut *con)
            .await?;

    Ok(res.unwrap_or(0))
}

pub async fn create_new_gen(transaction: &mut PgConnection, gen_id: i32) -> DbResult<()> {
    // info!("Creating new subjects generation {}", gen_id);
    sqlx::query!(
        "INSERT INTO subjects_generation (gen_id, creation_time) VALUES ($1, NOW())\
         ON CONFLICT DO NOTHING",
        gen_id
    )
    .execute(transaction)
    .await
    .context("Failed to insert new subjects generation")?;

    Ok(())
}

pub async fn get_subjects_for_group(
    con: &mut PgConnection,
    group_id: i32,
) -> DbResult<Vec<SubjectModel>> {
    let res = sqlx::query_as!(
        SubjectModel,
        "SELECT subjects.* FROM subjects join schedule_objs on \
            subjects.subject_id = schedule_objs.subject_id and subjects.gen_start <= schedule_objs.subject_gen_id \
            and (subjects.gen_end IS null OR subjects.gen_end > schedule_objs.subject_gen_id)\
        WHERE schedule_objs.gen_end IS NULL AND schedule_objs.group_id = $1 \
        GROUP BY subjects.subject_id, subject_obj_id",
        group_id)
        .fetch_all(&mut *con).await?;

    Ok(res)
}

pub async fn get_active_subject_by_id(
    subject_id: i32,
    con: &mut PgConnection,
) -> DbResult<Option<SubjectModel>> {
    let res = sqlx::query_as!(
        SubjectModel,
        "SELECT * FROM subjects WHERE subject_id = $1 AND gen_end IS NULL",
        subject_id
    )
    .fetch_optional(con)
    .await
    .context("Failed to fetch subject in subject merge")?;

    Ok(res)
}

use std::fmt::Debug;
use std::ops::FromResidual;
use anyhow::Context;
use reqwest::Response;
use rocket::serde::json::Json;
use serde_json::Value;

#[derive(serde::Deserialize, Debug)]
pub struct TimeResponse {
    pub time: String,
    pub week: i32,
}

fn route(query: &str) -> String {
    format!("https://digital.etu.ru/attendance/api/{}", query)
}
pub async fn get_time() -> anyhow::Result<TimeResponse> {
    let response: Response = reqwest::Client::new()
        .get(route("settings/time"))
        .send()
        .await?;

    let result: TimeResponse = response.json().await?;
    Ok(result)
}

#[derive(serde::Deserialize, Debug)]
pub struct LessonResponse {
    pub id: i32,
    pub title: String,
    pub shortTitle: String,
    pub subjectType: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub midname: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct LessonInstanceResponse {
    pub id: i32,
    pub start: String,
    pub end: String,
    pub isDistant: bool,
    pub room: Option<String>,
    pub lesson: LessonResponse,
    pub teachers: Vec<TeacherResponse>,

    pub selfReported: Option<bool>,
    pub groupLeaderReported: Option<bool>,
    pub teacherReported: Option<bool>,
    pub isGroupLeader: bool,
    pub checkInStart: String,
    pub checkInDeadline: String,
}

#[derive(Debug)]
pub enum GetScheduleResult {
    Ok(Vec<LessonInstanceResponse>),
    WrongToken,
    Error(anyhow::Error),
}

impl<T: Debug> FromResidual<Result<T, anyhow::Error>> for GetScheduleResult {
    fn from_residual(residual: Result<T, anyhow::Error>) -> Self {
        GetScheduleResult::Error(residual.unwrap_err())
    }
}

pub async fn get_cur_schedule(token: String) -> GetScheduleResult {
    let response: Response = reqwest::Client::new()
        .get(route("schedule/check-in"))
        .header("Cookie", format!("connect.digital-attendance={}", token))
        .send()
        .await.context("Cannot make fetch to schedule from etu attendance")?;

    info!("Result code: {:?}", response.status().as_u16());

    if response.status().is_success() {
        let result: Vec<LessonInstanceResponse> = response.json().await.context("Cannot parse get_cur_schedule result with success code as LessonInstanceResponse json!")?;
        GetScheduleResult::Ok(result)
    }
    else {
        warn!("Cannot get schedule: status code: {:?}", response.status().as_str());

        if response.status().as_u16() == 401 {
            return GetScheduleResult::WrongToken;
        }

        let result: Value = response.json().await.context("Cannot parse get_cur_schedule response as json")?;
        if let Ok(result) = serde_json::from_value::<AttendanceCheckInResponseError>(result.clone()) {
            match result.message.as_str() {
                _ => unimplemented!("Cannot parse error: {:?}", result)
            }
        } else {
            unimplemented!("Cannot parse error: {:?}", result)
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct AttendanceCheckInResponse {
    pub ok: bool,
}
#[derive(serde::Deserialize, Debug)]
pub struct AttendanceCheckInResponseError {
    pub message: String,
}

#[derive(Debug)]
pub enum CheckInResult {
    Ok,
    TooEarly,
    TooLate,
    WrongToken,
    Error(anyhow::Error),
}

impl<T: Debug> FromResidual<Result<T, anyhow::Error>> for CheckInResult {
    fn from_residual(residual: Result<T, anyhow::Error>) -> Self {
        CheckInResult::Error(residual.unwrap_err())
    }
}

pub async fn check_in(token: String, lesson_instance_id: i32) -> CheckInResult {
    let response: Response = reqwest::Client::new()
        .post(route(&format!("schedule/check-in/{}", lesson_instance_id)))
        .header("Cookie", format!("connect.digital-attendance={}", token))
        .send()
        .await.unwrap();

    let err_code = response.status().as_u16();
    if response.status().is_success() {
        let result: AttendanceCheckInResponse = response.json().await.context("Cannot parse check_in result with success code as json!")?;
        if result.ok {
            CheckInResult::Ok
        } else {
            CheckInResult::Error(anyhow::anyhow!("Cannot make check-in: ok is not true!"))
        }
    } else {
        warn!("Cannot make check-in: status code: {:?}", response.status().as_str());

        if err_code == 401 {
            return CheckInResult::WrongToken;
        }

        let result: Value = response.json().await.context("Cannot parse check_in response as json")?;
        if let Ok(result) = serde_json::from_value::<AttendanceCheckInResponseError>(result.clone()) {
            match result.message.as_str() {
                "Время для отметки истекло" => CheckInResult::TooLate,
                "Время для отметки ещё не наступило" => CheckInResult::TooEarly,
                "Не найдено" => CheckInResult::Error(anyhow::anyhow!("Cannot make check-in: lesson instance was not found!")),
                _ => unimplemented!("Cannot parse error: {:?}", result)
            }
        } else {
            unimplemented!("Cannot parse error: {:?}", result)
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct UserGroupResponse {
    pub role: String,
    pub status: String,
    pub isNew: bool,
    pub groupId: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct GroupResponse {
    pub name: String,
    pub isFake: bool,
    pub studyLevel: String,
    pub studyForm: String,
    pub UserGroup: UserGroupResponse,
}

#[derive(serde::Deserialize, Debug)]
pub struct UserResponse {
    pub initials: String,
    pub id: i32,
    pub surname: String,
    pub name: String,
    pub midname: String,
    pub email: String,
    pub lkId: i32,
    pub roles: Vec<String>,
    pub personalNumber: String,
    pub birthday: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub groups: Vec<GroupResponse>,
    pub curated: Vec<Value>,
    pub departments: Vec<Value>,
    pub faculties: Vec<Value>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CurrentUserResponse {
    pub user: Option<UserResponse>,
}

pub enum GetCurrentUserResult {
    Ok(UserResponse),
    WrongToken,
    Error(anyhow::Error),
}

impl<T: Debug> FromResidual<Result<T, anyhow::Error>> for GetCurrentUserResult {
    fn from_residual(residual: Result<T, anyhow::Error>) -> Self {
        GetCurrentUserResult::Error(residual.unwrap_err())
    }
}
pub async fn get_current_user(token: String) -> GetCurrentUserResult {
    let response: Response = reqwest::Client::new()
        .get(route("auth/current-user"))
        .header("Cookie", format!("connect.digital-attendance={}", token))
        .send()
        .await.context("Cannot make fetch to current user from etu attendance")?;

    let status_code = response.status().as_u16();

    let result: CurrentUserResponse = response.json().await.context("Cannot parse get_current_user result with success code as json!")?;
    if let Some(result) = result.user {
        GetCurrentUserResult::Ok(result)
    }
    else {
        return GetCurrentUserResult::WrongToken;
    }

}
use std::sync::Arc;

use actix_web::{post, web, Result as AwResult};
use chrono::{Utc, NaiveDateTime};
use maud::{html, Markup};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::rook_exercise::model::errors::AppError;
use crate::rook_exercise::model::exercise::Exercise;
use crate::rook_exercise::page::error_display::render_error;
use crate::rook_exercise::service::exercise_repo::ExerciseRepo;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseAddRequest {
    pub activity: String,
    pub description: String,
    pub start_dt: String,
    pub end_dt: String,
}

fn parse_datetime(field: impl Into<String>, dt_str: &str) -> Result<NaiveDateTime, AppError> {
    chrono::NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%dT%H:%M")
        .map_err(|e| AppError::InputError { field: field.into(), message: e.to_string() })
}

impl ExerciseAddRequest {
    pub fn to_model(&self) -> Result<Exercise, AppError> {
        Ok(Exercise {
            id: Uuid::new_v4().to_string(),
            user_id: "1".into(),
            activity: self.activity.clone(),
            description: self.description.clone(),
            start_dt: parse_datetime("start_dt", &self.start_dt)?,
            end_dt: parse_datetime("end_dt", &self.end_dt)?,
            created_dt: Utc::now(),
            modified_dt: Utc::now()
        })
    }
}

async fn add_process(
    exercise_repo: web::Data<Arc<dyn ExerciseRepo>>,
    request: web::Form<ExerciseAddRequest>) -> Result<(), AppError>
{
    println!("Have request: {:?}", &request);
    let exercise = request.to_model()?;
    let _ = exercise_repo.save(&exercise).await?;
    Ok(())
}

fn add_render() -> Markup {
    html! {
        div { "Saved" }
    }
}

#[post("/activity/add")]
pub async fn add(
    exercise_repo: web::Data<Arc<dyn ExerciseRepo>>,
    request: web::Form<ExerciseAddRequest>) -> AwResult<Markup>
{
    Ok(add_process(exercise_repo, request)
        .await
        .map(|_| add_render())
        .unwrap_or_else(render_error))
}
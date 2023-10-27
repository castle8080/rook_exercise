use std::sync::Arc;

use actix_web::{get, web, Result as AwResult};
use maud::{html, Markup};

use crate::rook_exercise::service::exercise_repo::ExerciseRepo;
use crate::rook_exercise::model::exercise::Exercise;
use crate::rook_exercise::model::errors::AppError;
use crate::rook_exercise::page::error_display::render_error;

async fn list_process(exercise_repo: &web::Data<Arc<dyn ExerciseRepo>>) -> Result<Vec<Exercise>, AppError> {
    exercise_repo.list("1").await
}

fn list_render(exercises: Vec<Exercise>) -> Markup {
    html! {
        div { "We have them: " (format!("items: {:?}", exercises)) }
    }
}

#[get("/activity/list")]
pub async fn list(exercise_repo: web::Data<Arc<dyn ExerciseRepo>>) -> AwResult<Markup> {
    Ok(list_process(&exercise_repo)
        .await
        .map(list_render)
        .unwrap_or_else(render_error))
}
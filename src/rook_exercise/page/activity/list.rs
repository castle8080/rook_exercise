use std::sync::Arc;

use actix_web::{get, web, Result as AwResult};
use maud::{html, Markup, Render};

use crate::rook_exercise::service::exercise_repo::ExerciseRepo;
use crate::rook_exercise::model::exercise::Exercise;
use crate::rook_exercise::model::errors::AppError;
use crate::rook_exercise::page::error_display::render_error;

async fn list_process(exercise_repo: &web::Data<Arc<dyn ExerciseRepo>>) -> Result<Vec<Exercise>, AppError> {
    exercise_repo.list("1").await
}

fn list_render(exercises: Vec<Exercise>) -> Markup {
    fn header(content: impl Render) -> Markup {
        html! { th { (content) } }
    }

    fn cell(content: impl Render) -> Markup {
        html! { td { (content) } }
    }

    // I want to figure out if I can manage CSS differently
    // where you can specify style closer to the generated content, but
    // not having to repeat it directly inline.

    html! {
        div class="activity_list" {
            h2 { "There are " (exercises.len()) " items." }
            table {
                tr {
                    (header("Id"))
                    (header("Activity"))
                    (header("Description"))
                    (header("Started"))
                    (header("Ended"))
                }
                @for e in &exercises {
                    tr {
                        (cell(&e.id))
                        (cell(&e.activity))
                        (cell(&e.description))
                        (cell(&e.start_dt.to_string()))
                        (cell(&e.end_dt.to_string()))
                    }
                }
            }
        }
    }
}

#[get("/activity/list")]
pub async fn list(exercise_repo: web::Data<Arc<dyn ExerciseRepo>>) -> AwResult<Markup> {
    Ok(list_process(&exercise_repo)
        .await
        .map(list_render)
        .unwrap_or_else(render_error))
}
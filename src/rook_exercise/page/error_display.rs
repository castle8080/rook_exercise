use maud::{html, Markup};

use crate::rook_exercise::model::errors::AppError;

pub fn render_error(error: AppError) -> Markup {
    html! {
        div {
            h2 { "Error Occurred: " (error) }
        }
    }
}

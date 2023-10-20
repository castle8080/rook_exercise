use actix_web::{get, Result as AwResult};
use maud::{html, Markup};

#[get("/stats")]
pub async fn stats() -> AwResult<Markup> {
    Ok(html! {
        "The stats are!"
    })
}
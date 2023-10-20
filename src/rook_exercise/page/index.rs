use actix_web::{get, Result as AwResult};
use maud::{html, Markup};

#[get("/")]
pub async fn index() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                title { "Rook Exercise" }
            }
            body {
                h1 { "Rook Exercise" }
                p {
                    "Log my exercise stuff!"
                }
            }
        }
    })
}
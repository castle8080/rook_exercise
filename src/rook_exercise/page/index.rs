use actix_web::{get, Result as AwResult};
use maud::{html, Markup};

#[get("/")]
pub async fn index() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                title { "Rook Exercise" }
                script src="/javascript/lib/htmx.min.js" { }
            }
            body {
                h1 { "Rook Exercise" }
                p id = "parent-div" {
                    "Log my exercise stuff!"
                }
                button
                    hx-get = "/stats"
                    hx-trigger = "click"
                    hx-target = "#parent-div"
                    hx-swap = "outerHTML"
                {
                    "Click Me!"
                }
            }
        }
    })
}
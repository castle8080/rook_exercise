use actix_web::{get, Result as AwResult};
use maud::{html, Markup};

#[get("/")]
pub async fn index() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                title { "Rook Exercise" }
                script src="/javascript/lib/htmx.min.js" { }
                link rel="stylesheet" href="/style.css" { }
            }
            body {
                h1 { "Exercise" }
                ul {
                    li {
                        a hx-get = "/activity/list"
                          hx-trigger = "click"
                          hx-target = "#main-content"
                          hx-swap = "innerHTML"
                          href = "#"
                        {
                            "Recent Activity"
                        }
                    }
                    li {
                        a hx-get = "/activity/add_form"
                          hx-trigger = "click"
                          hx-target = "#main-content"
                          hx-swap = "innerHTML"
                          href = "#"
                        {
                            "Add Activity"
                        }
                    }
                }
                div id = "main-content" {
                    ""
                }
            }
        }
    })
}
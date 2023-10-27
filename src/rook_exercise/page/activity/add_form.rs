use actix_web::{get, Result as AwResult};
use maud::{html, Markup};

#[get("/activity/add_form")]
pub async fn activity_add_form() -> AwResult<Markup> {
    Ok(html! {
        div {
            form
                method = "POST"
                action = "/activity/add"
            {
                div { h2 { "Add New Exercise Activity" } }
                br { }
                label {
                    span { "Activity Type" }
                    br { }
                    select name = "activity" {
                        option { "Run" }
                        option { "Walk" }
                        option { "Bicycle" }
                    }
                }
                br { }
                label {
                    span { "Description" }
                    br { }
                    input name = "description" type = "text" { }
                }
                br { }
                label {
                    span { "Started" }
                    br { }
                    input name = "start_dt" type = "datetime-local" { }
                }
                br { }
                label {
                    span { "Ended" }
                    br { }
                    input name = "end_dt" type = "datetime-local" { }
                }
                p { }
                input type = "Submit" value = "Save" { }
            }
        }
    })
}
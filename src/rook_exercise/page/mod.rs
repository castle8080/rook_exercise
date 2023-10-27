use actix_web::web;

mod index;
mod activity;
mod error_display;

pub fn configure_page_handlers(cfg: &mut web::ServiceConfig) {
    println!("Configuring page handlers");
    cfg
        .service(index::index)
        .service(activity::add::add)
        .service(activity::add_form::activity_add_form)
        .service(activity::list::list);
}

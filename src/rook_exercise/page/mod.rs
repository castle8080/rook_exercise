use actix_web::web;

mod index;
mod stats;

pub fn configure_page_handlers(cfg: &mut web::ServiceConfig) {
    println!("Configuring page handlers");
    cfg
        .service(index::index)
        .service(stats::stats);
}

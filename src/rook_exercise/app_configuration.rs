use actix_web::web;

use crate::rook_exercise::api::static_handler::create_static_handler;
use crate::rook_exercise::page::configure_page_handlers;

#[derive(Clone)]
pub struct AppConfiguration {
}

impl AppConfiguration {
    pub fn create_default() -> AppConfiguration {
        AppConfiguration {
        }
    }

    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        println!("Configuring all handlers");
        cfg.configure(configure_page_handlers);
        cfg.configure(create_static_handler);
    }
}

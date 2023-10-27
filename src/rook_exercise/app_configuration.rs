use std::sync::Arc;

use actix_web::web;

use crate::rook_exercise::api::static_handler::create_static_handler;
use crate::rook_exercise::model::errors::AppError;
use crate::rook_exercise::page::configure_page_handlers;
use crate::rook_exercise::service::exercise_repo::ExerciseRepo;
use crate::rook_exercise::service::exercise_repo_sled::ExerciseRepoSled;

#[derive(Clone)]
pub struct AppConfiguration {
    pub exercise_repo: Arc<dyn ExerciseRepo>,
}

impl AppConfiguration {
    pub fn create_default() -> Result<AppConfiguration, AppError> {
        let db = sled::open("var/exercise_db")?;
        Ok(AppConfiguration {
            exercise_repo: Arc::new(ExerciseRepoSled::create(db)?)
        })
    }

    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        println!("Configuring app data");
        cfg.app_data(web::Data::new(self.exercise_repo.clone()));
        
        println!("Configuring all handlers");
        cfg.configure(configure_page_handlers);
        cfg.configure(create_static_handler);
    }
}

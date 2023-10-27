use async_trait::async_trait;
use sled::{Db, Tree};

use crate::rook_exercise::model::exercise::Exercise;
use crate::rook_exercise::model::errors::AppError;
use crate::rook_exercise::service::exercise_repo::ExerciseRepo;

pub struct ExerciseRepoSled {
    pub db: Db
}

impl ExerciseRepoSled {
    pub fn create(db: Db) -> Result<Self, AppError> {
        Ok(ExerciseRepoSled { db })
    }

    fn open_table(&self, user_id: &String) -> Result<Tree, AppError> {
        // I kind of wish I could use a nested hierarchy for tree names.
        Ok(self.db.open_tree(format!("exercise:{user_id}"))?)
    }
}

#[async_trait]
impl ExerciseRepo for ExerciseRepoSled {

    async fn save(&self, exercise: &Exercise) -> Result<(), AppError> {
        let table = self.open_table(&exercise.user_id)?;
        let _ = table.insert(&exercise.id, serde_json::to_vec(exercise)?)?;
        Ok(())
    }

    async fn list(&self, user_id: &str) -> Result<Vec<Exercise>, AppError> {
        let user_id = user_id.to_string(); // stupid copy
        let table = self.open_table(&user_id)?;
        let mut entries: Vec<Exercise> = Vec::new();
        for pair in table.into_iter() {
            entries.push(serde_json::from_slice(&pair?.1)?);
        }
        Ok(entries)
    }
}
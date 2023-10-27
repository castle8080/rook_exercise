use async_trait::async_trait;

use crate::rook_exercise::model::exercise::Exercise;
use crate::rook_exercise::model::errors::AppError;

#[async_trait]
pub trait ExerciseRepo: Sync + Send {
    async fn save(&self, exercise: &Exercise) -> Result<(), AppError>;
    async fn list(&self, user_id: &str) -> Result<Vec<Exercise>, AppError>;
}
use sled;

use crate::rook_exercise::model::errors::AppError;

impl From<sled::Error> for AppError {
    fn from(e: sled::Error) -> Self {
        AppError::ServiceError(format!("Database error occurred: {e}"))
    }
}

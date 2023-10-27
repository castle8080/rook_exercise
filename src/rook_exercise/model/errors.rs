use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unable to authenticate: {0}")]
    AuthenticationError(String),

    #[error("Communication error: {0}")]
    CommunicationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Input error: {field} - {message}")]
    InputError { field: String, message: String }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerializationError(e.to_string())
    }
}

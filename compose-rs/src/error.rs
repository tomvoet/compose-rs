use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComposeError {
    #[error("Docker Compose file not found at {0}")]
    FileNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Command failed: {} - {}", .0.status, String::from_utf8_lossy(&.0.stderr))]
    CommandFailed(std::process::Output),
    #[error("Invalid Arguments: {0}")]
    InvalidArguments(String),
    #[error("Failed to Deserialize JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Failed to Parse docker output: {0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum ComposeBuilderError {
    #[error("Docker Compose file not found at {0}")]
    FileNotFound(String),
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

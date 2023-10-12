use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GameError {
    #[error("Failed to find player id | id: {0}")]
    PlayerNotFound(i32),
}

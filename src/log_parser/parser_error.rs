use thiserror::Error;

use crate::{game::GameError, TournamentError};

// Errors related to game log parsing.
#[derive(Debug, Error)]
pub enum LogParserError {
    #[error("Failed to process event | description: Game has not yet been initialized")]
    GameNotInitialized,
    #[error("Failed to process event | description: {0}")]
    ProcessEvent(GameError),
    #[error("Failed to process event | description: {0}")]
    TournamentError(TournamentError),
}

impl From<GameError> for LogParserError {
    // Converts a game error into a `LogParserError`.
    fn from(game_error: GameError) -> Self {
        LogParserError::ProcessEvent(game_error)
    }
}

impl From<TournamentError> for LogParserError {
    // Converts a tournament error into a `LogParserError`.
    fn from(tournament_error: TournamentError) -> Self {
        LogParserError::TournamentError(tournament_error)
    }
}

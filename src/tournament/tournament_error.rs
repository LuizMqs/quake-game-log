use thiserror::Error;

#[derive(Error, Debug)]
pub enum TournamentError {
    #[error("The tournament has not started yet!")]
    TournamentNotStarted,
}

use crate::{game::Game, Tournament};

use super::{game_event::QuakeEvent, parser_error::LogParserError};

pub struct QuakeLogParser {}

impl QuakeLogParser {
    pub fn new() -> Self {
        Self {}
    }

    // Parses the Quake log and returns a Result containing the Tournament or a LogParserError.
    pub fn parse_log(&self, quake_log: String) -> Result<Tournament, LogParserError> {
        let mut tournament = Tournament::new();

        let mut current_game: Option<Game> = None;

        for line in quake_log.lines() {
            let event = QuakeEvent::from_log_line(line.to_string());

            if let QuakeEvent::InitGame = event {
                if let Some(game) = &current_game {
                    tournament.tournament_handler(game.clone())?;
                }

                current_game = Some(Game::new());
            } else if let QuakeEvent::ChangePlayerInfo {
                player_id,
                new_name,
            } = event
            {
                if let Some(ref mut game) = current_game {
                    game.update_player_list_by_id(player_id, new_name)
                } else {
                    return Err(LogParserError::GameNotInitialized);
                }
            } else if let QuakeEvent::PlayerKill {
                killer_id,
                victim_id,
                cause,
            } = event
            {
                if let Some(ref mut game) = current_game {
                    game.handle_kill_event(killer_id, victim_id, &cause)?;
                } else {
                    return Err(LogParserError::GameNotInitialized);
                }
            }
        }

        if let Some(game) = current_game {
            tournament.tournament_handler(game.clone())?;
        }

        Ok(tournament)
    }
}

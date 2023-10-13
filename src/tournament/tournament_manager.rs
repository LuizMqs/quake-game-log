use std::collections::HashMap;

use crate::game::Game;

use super::tournament_error::TournamentError;

pub struct Tournament {
    pub games: Vec<Game>,
    pub ranking: HashMap<i32, (String, i32)>,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
            ranking: HashMap::new(),
        }
    }

    pub fn add_game_to_tournament(&mut self, game: Game) {
        self.games.push(game);
    }

    // Updates the players' ranking in the tournament by id and changes it to the last name that id used
    pub fn update_the_ranking(&mut self) -> Result<(), TournamentError> {
        for game in &self
            .games
            .last()
            .ok_or(TournamentError::TournamentNotStarted)
        {
            for (id, player) in &game.players {
                // Checks if the player is already registered in the tournament using id or additional

                let (name, total_kills) = self
                    .ranking
                    .entry(id.clone())
                    .or_insert((player.name.to_string(), 0));

                // Changes the player's name to the last one that that same id used
                *name = player.name.clone();
                *total_kills += player.kills;
            }
        }

        Ok(())
    }

    pub fn tournament_handler(&mut self, game: Game) -> Result<(), TournamentError> {
        self.add_game_to_tournament(game);
        self.update_the_ranking()?;

        Ok(())
    }
}

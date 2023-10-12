use std::collections::HashMap;

use crate::{player::Player, GameError};

#[derive(PartialEq, Debug, Clone)]
pub struct Game {
    pub total_kills: i32,
    pub players: HashMap<i32, Player>,
    pub death_causes: HashMap<String, i32>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            total_kills: 0,
            players: HashMap::new(),
            death_causes: HashMap::new(),
        }
    }

    pub fn add_kill_to_game(&mut self) {
        self.total_kills += 1;
    }

    pub fn add_kill_to_player_by_id(&mut self, id: i32) {
        // Impossible to throw error because of previous validation
        let player = self.players.get_mut(&id).expect("Player not was found!");

        player.killed_another_player();
        self.add_kill_to_game();
    }

    pub fn subtract_kill_to_player_by_id(&mut self, id: i32) {
        // Impossible to throw error because of previous validation
        let player = self.players.get_mut(&id).expect("Player not was found!");

        player.death_around_the_world();
        self.add_kill_to_game();
    }

    pub fn update_player_list_by_id(&mut self, player_id: i32, new_name: String) {
        let player: &mut Player = self
            .players
            .entry(player_id)
            .or_insert_with_key(|_| Player::new(new_name.clone()));

        player.changed_name(new_name);
    }

    pub fn validate_player_id(&self, id: i32) -> Result<(), GameError> {
        // id 1022 is always validated as it corresponds to the world
        if id == 1022 {
            return Ok(());
        }

        self.players
            .contains_key(&id)
            .then(|| {})
            .ok_or(GameError::PlayerNotFound(id))
    }

    // Handles death events, updates the appropriate counters and cause of death.
    pub fn handle_kill_event(
        &mut self,
        killer_id: i32,
        victim_id: i32,
        cause: &str,
    ) -> Result<(), GameError> {
        self.validate_player_id(killer_id)?;
        self.validate_player_id(victim_id)?;

        if killer_id == 1022 {
            self.subtract_kill_to_player_by_id(victim_id)
        } else if killer_id != victim_id {
            self.add_kill_to_player_by_id(killer_id)
        } else {
            self.add_kill_to_game();
        }

        let cause_count = self.death_causes.entry(cause.to_string()).or_insert(0);
        *cause_count += 1;

        Ok(())
    }
}

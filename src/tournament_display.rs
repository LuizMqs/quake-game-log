use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

use crate::tournament::Tournament;

pub struct TournamentDisplay {
    tournament: Tournament,
}

impl TournamentDisplay {
    pub fn new(tournament: Tournament) -> Self {
        Self { tournament }
    }

    pub fn display_grouped_games(&self) -> io::Result<()> {
        let mut log_file = File::create("data/game_log.txt")?;

        for (i, game) in self.tournament.games.iter().enumerate() {
            let mut players_game: HashMap<String, i32> = HashMap::new();

            writeln!(&mut log_file, "\"game_{}\": {{", i + 1)?;
            writeln!(&mut log_file, "\t\"total_kills\": {},", game.total_kills)?;

            let players: Vec<&String> = game.players.values().map(|player| &player.name).collect();
            writeln!(&mut log_file, "\t\"players\": {:?},", players)?;

            for (_, player) in &game.players {
                players_game.insert(player.name.clone(), player.kills);
            }

            writeln!(&mut log_file, "\t\"kills\": {{")?;

            let mut sorted_players: Vec<(&String, &i32)> = players_game.iter().collect();
            sorted_players.sort_by(|a, b| b.1.cmp(a.1));

            for (player_name, kills) in sorted_players {
                writeln!(&mut log_file, "\t\t\"{}\": {},", player_name, kills)?;
            }
            writeln!(&mut log_file, "\t}},")?;

            let mut sorted_death_causes: Vec<(&String, &i32)> = game.death_causes.iter().collect();
            sorted_death_causes.sort_by(|a, b| b.1.cmp(a.1));

            writeln!(&mut log_file, "\t\"kills_by_means\": {{")?;

            for (cause, count) in sorted_death_causes {
                writeln!(&mut log_file, "\t\t\"{}\": {},", cause, count)?;
            }
            writeln!(&mut log_file, "\t}}")?;

            writeln!(&mut log_file, "}}")?;
        }

        Ok(())
    }

    pub fn display_tournament_results(&self) -> io::Result<()> {
        let mut log_file = File::create("data/players_ranking.txt")?;
        writeln!(&mut log_file, "\"tournament_results\": {{")?;
        writeln!(&mut log_file, "\t\"players\": {{")?;

        let mut sorted_players: Vec<(i32, &String, &i32)> = self
            .tournament
            .ranking
            .iter()
            .map(|(id, (name, kills))| (*id, name, kills))
            .collect();

        sorted_players.sort_by(|a, b| b.2.cmp(a.2));

        for (id, player_name, kills) in sorted_players {
            writeln!(
                &mut log_file,
                "\t\t\"ID {}: {}\": {},",
                id, player_name, kills
            )?;
        }

        writeln!(&mut log_file, "\t}}")?;
        writeln!(&mut log_file, "}}")?;

        Ok(())
    }
}

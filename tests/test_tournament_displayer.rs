#[cfg(test)]
mod tests {
    // Creates or adds test results to files
    use std::collections::HashMap;

    use quake_game_log::game::Game;
    use quake_game_log::tournament::Tournament;
    use quake_game_log::tournament_display::TournamentDisplay;

    #[test]
    fn test_tournament_displayer_games_should_show_grouped_games() {
        let mut tournament: Tournament = Tournament::new();
        let mut game1 = Game::new();
        let mut game2 = Game::new();

        game1.update_player_list_by_id(1, "Player1".to_string());
        let _ = game1.handle_kill_event(1022, 1, "MOD_TRIGGER_HURT");

        game2.update_player_list_by_id(1, "Player1".to_string());
        game2.update_player_list_by_id(2, "Player2".to_string());

        assert!(tournament.tournament_handler(game1).is_ok());
        assert!(tournament.tournament_handler(game2).is_ok());

        let display_results = TournamentDisplay::new(tournament);
        let result = display_results.display_grouped_games();

        assert!(result.is_ok());
    }

    #[test]
    fn test_tournament_displayer_should_show_grouped_names() {
        let mut tournament = Tournament {
            games: Vec::new(),
            ranking: HashMap::new(),
        };

        tournament.ranking.insert(1, ("Player1".to_string(), 10));
        tournament.ranking.insert(2, ("Player2".to_string(), 15));
        tournament.ranking.insert(3, ("Player3".to_string(), 5));

        let displayer = TournamentDisplay::new(tournament);

        assert!(displayer.display_tournament_results().is_ok());
    }
}

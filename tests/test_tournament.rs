#[cfg(test)]
mod tests {
    use quake_game_log::{
        game::Game,
        player::Player,
        tournament::{Tournament, TournamentError},
    };

    #[test]
    fn test_tournament_new() {
        let tournament = Tournament::new();
        assert!(tournament.games.is_empty());
        assert!(tournament.ranking.is_empty());
    }

    #[test]
    fn test_tournament_add_game_to_tournament_should_include_existing_games() {
        let mut tournament = Tournament::new();
        let mut game = Game::new();
        game.players.insert(1, Player::new("Player1".to_string()));
        game.add_kill_to_player_by_id(1);

        tournament.add_game_to_tournament(game.clone());
        assert_eq!(tournament.games, vec![game]);
    }

    #[test]
    fn test_pass_two_games_should_tournament_update_the_ranking() -> Result<(), TournamentError> {
        let mut tournament = Tournament::new();
        let mut game1 = Game::new();
        game1.players.insert(1, Player::new("Player1".to_string()));
        game1.add_kill_to_player_by_id(1);
        tournament.add_game_to_tournament(game1.clone());
        tournament.update_the_ranking()?;

        let mut game2 = Game::new();
        game2.players.insert(1, Player::new("Player2".to_string()));
        game2.add_kill_to_player_by_id(1);
        tournament.add_game_to_tournament(game2.clone());
        tournament.update_the_ranking()?;

        assert_eq!(
            tournament.ranking.get(&1),
            Some(&("Player2".to_string(), 2))
        );

        Ok(())
    }

    #[test]
    fn test_pass_a_game_should_contain_a_game() -> Result<(), TournamentError> {
        let mut tournament = Tournament::new();
        let game = Game::new();
        tournament.tournament_handler(game)?;

        assert_eq!(tournament.games.len(), 1);

        Ok(())
    }
}

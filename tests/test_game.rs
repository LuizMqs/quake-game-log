#[cfg(test)]
mod tests {
    use quake_game_log::game::Game;
    use quake_game_log::game::GameError;
    use quake_game_log::player::Player;

    #[test]
    fn test_new_game_should_have_zero_players() {
        let game = Game::new();

        assert_eq!(game.total_kills, 0);
        assert_eq!(game.players.len(), 0);
    }

    #[test]
    fn test_add_kill_to_game_should_add_one_kill_to_total_kills() {
        let mut game: Game = Game::new();
        game.add_kill_to_game();

        assert_eq!(game.total_kills, 1);
    }

    #[test]
    fn test_add_kill_to_player_by_id_player_found() {
        let mut game = Game::new();
        game.players.insert(1, Player::new("Player1".to_string()));
        game.add_kill_to_player_by_id(1);

        assert_eq!(game.players.get(&1).unwrap().kills, 1);
        assert_eq!(game.total_kills, 1);
    }

    #[test]
    #[should_panic(expected = "Player not was found!")]
    fn test_add_kill_to_player_by_id_expect_panic() {
        let mut game = Game::new();

        game.add_kill_to_player_by_id(1);
    }

    #[test]
    fn test_add_kill_to_a_player_and_is_expected_to_get() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());
        game.update_player_list_by_id(2, "Player2".to_string());

        game.add_kill_to_player_by_id(1);

        assert_eq!(game.players.get(&1).unwrap().kills, 1);
        assert_eq!(game.total_kills, 1);
        assert_eq!(game.players.get(&2).unwrap().kills, 0);
    }

    #[test]
    fn test_subtract_player_kill_should_decrease_and_be_greater_than_zero() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());

        assert_eq!(game.players.get(&1).unwrap().kills, 0);
        assert_eq!(game.total_kills, 0);

        game.subtract_kill_to_player_by_id(1);

        assert_eq!(game.players.get(&1).unwrap().kills, 0);
        assert_eq!(game.total_kills, 1);

        game.add_kill_to_player_by_id(1);
        game.add_kill_to_player_by_id(1);

        assert_eq!(game.players.get(&1).unwrap().kills, 2);
        assert_eq!(game.total_kills, 3);

        game.subtract_kill_to_player_by_id(1);

        assert_eq!(game.players.get(&1).unwrap().kills, 1);
        assert_eq!(game.total_kills, 4);
    }

    #[test]
    #[should_panic(expected = "Player not was found!")]
    fn test_subtract_kill_to_player_by_id_expect_panic() {
        let mut game = Game::new();

        game.subtract_kill_to_player_by_id(1);
    }

    #[test]
    fn test_send_name_and_id_should_create_player_or_update_one_with_that_id() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());
        game.update_player_list_by_id(1, "ChangedName".to_string());
        game.update_player_list_by_id(2, "Player2".to_string());
        assert_eq!(
            game.players.get(&1).unwrap().name,
            "ChangedName".to_string()
        );
        assert_eq!(game.players.get(&2).unwrap().name, "Player2".to_string());
    }

    #[test]
    fn test_send_id_should_return_that_it_exists() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());
        game.update_player_list_by_id(2, "Player2".to_string());
        let result = game.validate_player_id(1);

        assert!(result.is_ok());

        let result = game.validate_player_id(2);

        assert!(result.is_ok());

        let result = game.validate_player_id(1022);

        assert!(result.is_ok());
    }

    #[test]
    fn test_send_a_nonexistent_id_should_return_an_error() {
        let mut game = Game::new();

        game.update_player_list_by_id(1, "Player1".to_string());
        let result = game.validate_player_id(2);

        assert_eq!(result, Err(GameError::PlayerNotFound(2)));
    }

    #[test]
    fn test_handling_a_kill_events_should_add_death_means_and_kills() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());
        game.update_player_list_by_id(2, "Player2".to_string());

        assert_eq!(game.handle_kill_event(1, 2, "MOD_TRIGGER_HURT"), Ok(()));

        assert_eq!(game.players.get(&1).unwrap().kills, 1);
        assert_eq!(game.total_kills, 1);
        assert_eq!(game.death_causes.get("MOD_TRIGGER_HURT"), Some(&1));
    }

    #[test]
    fn test_makes_player_to_kill_victim_nonexistent_should_return_error() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());

        assert_eq!(
            game.handle_kill_event(1, 2, "MOD_TRIGGER_HURT"),
            Err(GameError::PlayerNotFound(2))
        );
    }

    #[test]
    fn test_makes_nonexistent_player_kill_should_return_error() {
        let mut game = Game::new();
        game.update_player_list_by_id(2, "Player2".to_string());

        assert_eq!(
            game.handle_kill_event(1, 2, "MOD_TRIGGER_HURT"),
            Err(GameError::PlayerNotFound(1))
        );
    }

    #[test]
    fn test_handling_kill_events_with_nonexistent_killer_and_victim_should_return_error() {
        let mut game = Game::new();

        assert_eq!(
            game.handle_kill_event(1, 2, "MOD_TRIGGER_HURT"),
            Err(GameError::PlayerNotFound(1))
        );
    }

    #[test]
    fn test_player_kills_himself_it_should_that_his_kills_will_not_increase_only_those_in_the_game()
    {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());

        assert_eq!(game.handle_kill_event(1, 1, "MOD_TRIGGER_HURT"), Ok(()));

        assert_eq!(game.players.get(&1).unwrap().kills, 0);
        assert_eq!(game.total_kills, 1);
    }

    #[test]
    fn test_world_is_the_killer_should_the_victim_to_lose_a_kill() {
        let mut game = Game::new();
        game.update_player_list_by_id(1, "Player1".to_string());
        game.update_player_list_by_id(2, "Player2".to_string());

        assert_eq!(game.handle_kill_event(1022, 1, "MOD_TRIGGER_HURT"), Ok(()));

        assert_eq!(game.players.get(&1).unwrap().kills, 0);
        assert_eq!(game.total_kills, 1);

        assert_eq!(game.handle_kill_event(1, 2, "MOD_TRIGGER_HURT"), Ok(()));

        assert_eq!(game.players.get(&1).unwrap().kills, 1);
        assert_eq!(game.total_kills, 2);

        assert_eq!(game.handle_kill_event(1022, 1, "MOD_TRIGGER_HURT"), Ok(()));

        assert_eq!(game.players.get(&1).unwrap().kills, 0);
        assert_eq!(game.total_kills, 3);
    }
}

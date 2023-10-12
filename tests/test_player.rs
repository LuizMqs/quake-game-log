#[cfg(test)]
mod tests {
    use quake_game_log::player::Player;

    #[test]
    fn test_new_player() {
        let player = Player::new("Alice".to_string());
        assert_eq!(player.name, "Alice");
        assert_eq!(player.kills, 0);
    }

    #[test]
    fn test_killed_another_player() {
        let mut player = Player::new("Bob".to_string());
        player.killed_another_player();
        assert_eq!(player.kills, 1);
    }

    #[test]
    fn test_death_around_the_world() {
        let mut player = Player::new("Charlie".to_string());
        player.kills = 3;
        player.death_around_the_world();
        assert_eq!(player.kills, 2);
    }

    #[test]
    fn test_changed_name() {
        let mut player = Player::new("David".to_string());
        player.changed_name("Eve".to_string());
        assert_eq!(player.name, "Eve");
    }
}

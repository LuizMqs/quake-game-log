#[cfg(test)]
mod tests {
    use quake_game_log::log_parser::QuakeEvent;

    #[test]
    fn test_quake_event_from_log_line_init_game() {
        let line = r" 20:37 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\bot_minplayers\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0".to_string();
        let event = QuakeEvent::from_log_line(line);

        assert_eq!(event, QuakeEvent::InitGame);
    }

    #[test]
    fn test_quake_event_from_log_line_change_player_info() {
        let line = r" 20:34 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0".to_string();
        let event = QuakeEvent::from_log_line(line);

        assert_eq!(
            event,
            QuakeEvent::ChangePlayerInfo {
                player_id: 2,
                new_name: "Isgalamido".to_string()
            }
        );
    }

    #[test]
    fn test_quake_event_from_log_line_player_kill() {
        let line =
            r" 21:07 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT".to_string();
        let event = QuakeEvent::from_log_line(line);

        assert_eq!(
            event,
            QuakeEvent::PlayerKill {
                killer_id: 1022,
                victim_id: 2,
                cause: "MOD_TRIGGER_HURT".to_string()
            }
        );
    }

    #[test]
    fn test_quake_event_from_log_line_unknown() {
        let line = r" 20:40 Item: 2 weapon_rocketlauncher".to_string();
        let event = QuakeEvent::from_log_line(line);

        assert_eq!(
            event,
            QuakeEvent::Unknown(r" 20:40 Item: 2 weapon_rocketlauncher".to_string())
        );
    }
}

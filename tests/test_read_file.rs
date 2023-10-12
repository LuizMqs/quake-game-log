#[cfg(test)]
mod tests {
    use quake_game_log::read_file::LogReader;

    #[test]
    fn test_find_and_read_a_file_should_transform_it_into_a_string() {
        let mut file = LogReader::new("data/test_qgames.log".to_string());
        file.read_log_file().expect("Failed to read log file");

        let expect_log_data = r#"  0:00 ------------------------------------------------------------
  0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
 15:00 Exit: Timelimit hit.
 20:34 ClientConnect: 2
 20:34 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
 20:37 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
 20:37 ClientBegin: 2
 20:37 ShutdownGame:
"#.to_string();

        assert_eq!(file.log_data, expect_log_data)
    }
}

use quake_game_log::{
    log_parser::QuakeLogParser, read_file::LogReader, tournament_display::TournamentDisplay,
};

fn main() -> eyre::Result<()> {
    let log_file_path = "data/qgames.log";

    let mut file = LogReader::new(log_file_path.to_string());
    file.read_log_file().expect("Failed to read log file");

    let parser = QuakeLogParser::new();
    let parsed_data = parser.parse_log(file.log_data)?;

    let display = TournamentDisplay::new(parsed_data);
    display.display_grouped_games()?;
    display.display_tournament_results()?;

    Ok(())
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INIT_GAME_REGEX: Regex = Regex::new(r"InitGame: .+").expect("");
    static ref CHANGE_PLAYER_INFO_REGEX: Regex =
        Regex::new(r"ClientUserinfoChanged: ([0-9]+) n\\([A-z\s]+)\\t").expect("");
    static ref PLAYER_KILL: Regex =
        Regex::new(r"Kill: (\d+) (\d+) (\d+): (.+) killed (.+) by (.+)").expect("");
}

// Represents different Quake events that can occur in the game log.
#[derive(Debug, PartialEq)]
pub enum QuakeEvent {
    InitGame,
    ChangePlayerInfo {
        player_id: i32,
        new_name: String,
    },
    PlayerKill {
        killer_id: i32,
        victim_id: i32,
        cause: String,
    },
    Unknown(String),
}

impl QuakeEvent {
    // Parses a log line and returns the corresponding Quake event.

    pub fn from_log_line(line: String) -> Self {
        match (
            INIT_GAME_REGEX.captures(&line),
            CHANGE_PLAYER_INFO_REGEX.captures(&line),
            PLAYER_KILL.captures(&line),
        ) {
            (Some(_), ..) => Self::InitGame,
            (_, Some(capture), _) => Self::ChangePlayerInfo {
                player_id: capture[1].parse().expect("player_id should be integer"),
                new_name: capture[2].to_string(),
            },
            (.., Some(capture)) => Self::PlayerKill {
                killer_id: capture[1].parse().expect("killer_id should be integer"),
                victim_id: capture[2].parse().expect("victim_id should be integer"),
                cause: capture[6].parse().expect("cause should be string"),
            },
            _ => Self::Unknown(line),
        }
    }
}

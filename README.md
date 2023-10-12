# Quake Game Log

This project is designed to parse Quake game log files, perform various data analysis tasks, and generate reports based on the game data.

## Features

- Parse Quake log files with ease.
- Group game data for each match.
- Collect kill data, including player names and causes of death.
- Generate reports for each match and player rankings.

## Log Parsing

- Reading log files generated by a Quake 3 Arena server.
- Grouping game data for each match.
- Collecting kill data, including player and cause of death.
- Handling special cases like <world> kills and suicides.
- Calculating the total kills for each match.
- Report Generation
- The report generator creates informative reports, including:

## Report Generation

- Match reports with details like total kills and individual player kills.
- Player ranking reports based on player performance.
- Additional reports showing deaths grouped by cause for each match.

## How to Use

1. Clone this repository:

   ```shell
   git clone https://github.com/LuizMqs/quake-game-log.git
   cd quake_game_log
   ```

2. Build Project

   ```shell
    docker build -t quake-game-log .
   ```

3. Run Code

### Linux

```bash
    docker run -v $(pwd):/app meu-projeto-rust .
```

### Windows

```shell
    docker run -v ${PWD}:/app meu-projeto-rust .
```

**The files containing the results will be saved in the data folder**
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Debug)]
pub struct LogReader {
    pub log_file_path: String,
    pub log_data: String,
}

impl LogReader {
    pub fn new(log_file_path: String) -> Self {
        Self {
            log_file_path,
            log_data: String::new(),
        }
    }

    pub fn read_log_file(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.log_file_path)?;
        let reader = io::BufReader::new(file);
        let mut log_data = String::new();

        for line in reader.lines() {
            log_data.push_str(&line?);
            log_data.push('\n');
        }

        self.log_data = log_data;

        Ok(())
    }
}

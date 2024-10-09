use chrono::{NaiveDateTime, ParseError};
use std::fs::File;
use std::io::{BufRead, BufReader};
/// @psidh
//
#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub timestamp: NaiveDateTime,
}

impl Task {
    pub fn from_line(line: &str) -> Result<Self, ParseError> {
        let substrings: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

        let id = substrings[0].parse::<i32>().expect("Failed to parse id");
        let title = substrings[1].to_string();
        let description = substrings[2].to_string();
        let date = NaiveDate::parse_from_str(substrings[3], "%Y-%m-%d")?;
        let timestamp = NaiveDateTime::parse_from_str(substrings[4], "%Y-%m-%d %H:%M:%S")?;

        Ok(Task {
            id,
            title,
            description,
            date,
            timestamp,
        })
    }

    pub fn display(&self) {
        println!("Task ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Date: {}", self.date);
        println!("Timestamp: {}", self.timestamp);
        println!("-------------------");
    }
}

pub fn view_all_tasks(file_path: &str) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        match Task::from_line(&line) {
            Ok(task) => task.display(),
            Err(_) => println!("Error parsing line: {}", line),
        }
    }
}

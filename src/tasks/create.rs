use chrono::{NaiveDate, Utc};
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub struct Task {
    id: i32,
    title: String,
    description: String,
    date: NaiveDate,
}

impl Task {
    pub fn new(id: i32, title: String, description: String, date: NaiveDate) -> Self {
        Task {
            id,
            title,
            description,
            date,
        }
    }

    pub fn display(&self) {
        println!("Task created successfully");
        println!("Task ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Date: {}", self.date);
    }
    pub fn write_to_file(&self) {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let file_path = "db/tasks.txt";

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open file");

        writeln!(
            file,
            "{} | {} | {} | {} | {}",
            self.id, self.title, self.description, self.date, timestamp
        )
        .expect("Failed to write to file");
    }
}

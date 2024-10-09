use chrono::{NaiveDate, Utc};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
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

    // Write task to file
    pub fn write_to_file(&self) {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let file_path = "db/tasks.txt";

        // Ensure the db directory exists
        let db_path = Path::new("db");
        if !db_path.exists() {
            create_dir_all(db_path).expect("Failed to create db directory");
        }

        // Open the file and append the task
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

mod tasks;
mod view;

use chrono::NaiveDate;
use std::io::{self, Write};
use tasks::create::Task;
use view::view::{view_all_tasks, view_latest_tasks};

fn main() {
    println!("Welcome to CLI TASK Tracker");

    loop {
        println!();
        println!("Enter 1 to create a task");
        println!("Enter 2 to view all tasks");
        println!("Enter 3 to view the top 5 latest tasks");
        println!("Enter 0 to exit");
        print!("Your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            0 => {
                let mut response = String::new();
                print!("Are you sure you want to exit? (y/n): ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read line");
                if response.trim().to_lowercase() == "y" {
                    println!("Goodbye!");
                    break;
                }
            }
            1 => {
                let mut id = String::new();
                let mut title = String::new();
                let mut description = String::new();
                let mut date = String::new();

                print!("Enter task ID: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: i32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                print!("Enter task title: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");

                print!("Enter task description: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");

                print!("Enter task date (YYYY-MM-DD): ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut date)
                    .expect("Failed to read line");
                let date = match NaiveDate::parse_from_str(date.trim(), "%Y-%m-%d") {
                    Ok(date) => date,
                    Err(_) => {
                        println!(
                            "Invalid date format. Please enter the date in YYYY-MM-DD format."
                        );
                        continue;
                    }
                };

                let task = Task::new(
                    id,
                    title.trim().to_string(),
                    description.trim().to_string(),
                    date,
                );
                task.write_to_file(); // Write task to db/tasks.txt
            }
            2 => {
                println!("View all tasks:");
                view_all_tasks("db/tasks.txt"); // Display all tasks
            }
            3 => {
                println!("View the top 5 latest tasks:");
                view_latest_tasks("db/tasks.txt"); // Display top 5 latest tasks
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 0.");
            }
        }
    }
}

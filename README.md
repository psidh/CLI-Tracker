# 📝 CLI Tracker

**CLI Task Tracker** is a simple, interactive command-line tool for creating and managing tasks, developed in Rust.

## ⚡ Features

- Create tasks with title, description, and due date.
- Tasks are saved in `task.txt` with timestamps.
- View the top 5 most recent tasks.
- View all tasks stored.

## 📂 Project Structure

```bash
.
├── Cargo.toml
├── src/
│   ├── main.rs          # Main logic and user interaction
│   ├── tasks/           # Task creation and saving logic
│   │   └── tasks.rs
│   ├── view/            # View tasks from the file
│   │   └── view.rs
└── db/
    └── task.txt         # Stores tasks with timestamps
```

## 🚀 Getting Started

### Prerequisites
- Install **[Rust](https://www.rust-lang.org/tools/install)**.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/cli-task-tracker.git
   cd cli-task-tracker
   ```

2. Build and run the project:
   ```bash
   cargo build
   cargo run
   ```

## 🛠️ Usage

Upon running, the application presents a simple menu:

- `1`: **Create a Task**
  Input ID, title, description, and due date. The task is saved in `task.txt`.

- `2`: **View Latest Tasks**
  Displays the top 5 recent tasks.

- `3`: **View All Tasks**
  Lists all tasks created.

- `0`: **Exit**
  Quit the application.

### Example Task File (`db/task.txt`)

```txt
1 | Build Rust app | Complete the CLI project | 2024-10-01 | 2024-10-01 12:34:56
2 | Write README  | Document project usage   | 2024-10-08 | 2024-10-08 14:00:00
```

## 🚧 Future Enhancements

- Edit and delete tasks.
- Task sorting and filtering.

## 📜 License

This project is licensed under the **MIT License**.

---

This version keeps things concise, includes an example task file, and uses modern markdown elements like icons to make it visually appealing.

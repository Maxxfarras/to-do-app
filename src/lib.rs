pub use config::Config;
pub use database::Database;
pub use task::Task;

pub mod config {
    #[derive(Debug)]
    pub struct Config {
        pub command: String,
        pub argument: Option<String>,
    }

    impl Config {
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
            args.next();

            let command = match args.next() {
                Some(arg) => arg,
                None => return Err("No command parsed!!"),
            };

            let argument = args.next();

            Ok(Self { command, argument })
        }
    }
}

pub mod task {
    use crate::config::Config;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task {
        pub description: String,
        pub done: bool,
    }

    impl Task {
        pub fn build(config: Config) -> Result<Self, &'static str> {
            let description = match config.argument {
                Some(arg) => arg,
                None => return Err("No argument provided"),
            };

            let done = false;

            Ok(Self { description, done })
        }
    }
}

pub mod database {
    use crate::task::Task;
    use std::fs;

    #[derive(Debug)]
    pub struct Database {
        pub tasks: Vec<Task>,
    }

    impl Database {
        pub fn add(&mut self, task: Task) {
            self.tasks.push(task);
        }

        pub fn list(&self) {
            println!("Tasks to do:");

            let tasks_iter = self.tasks.iter();

            for i in tasks_iter {
                println!("[{}] {}", i.done, i.description);
            }
        }

        pub fn load(file_path: &str) -> Self {
            let contents = fs::read_to_string(file_path).unwrap_or_else(|_| "[]".to_string());
            let tasks = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
            Database { tasks }
        }

        pub fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
            let json = serde_json::to_string_pretty(&self.tasks).unwrap();
            fs::write(file_path, json)
        }
    }
}

use serde::{Deserialize, Serialize};
use std::{env, fs, process};

fn main() {
    let file_path = "tasks.json";
    let mut database = Database {
        tasks: load_database(file_path),
    };

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing the commands: {err}");
        process::exit(1);
    });

    let task = Task::build(config).unwrap_or_else(|err| {
        eprintln!("Error creating the task: {err}");
        process::exit(1);
    });

    database.add(task);
    database.list();

    save_database(&database.tasks, file_path).expect("Error saving database!");
}

#[derive(Debug)]
struct Config {
    command: String,
    argument: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
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

#[derive(Debug)]
struct Database {
    tasks: Vec<Task>,
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
}

fn load_database(file_path: &str) -> Vec<Task> {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_database(vector: &Vec<Task>, file_path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(vector).unwrap();
    fs::write(file_path, json)
}

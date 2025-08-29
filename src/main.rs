use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing the commands: {err}");
        process::exit(1);
    });

    let task1 = Task::build(config).unwrap_or_else(|err| {
        eprintln!("Error creating the task: {err}");
        process::exit(1);
    });

    let task2 = Task::build(Config {
        command: "add".to_string(),
        argument: Some("Clean Dishes".to_string()),
    })
    .unwrap(); // unwrap for simplicity in testing

    let task3 = Task::build(Config {
        command: "add".to_string(),
        argument: Some("Buy Milk".to_string()),
    })
    .unwrap();

    let mut database = Database { tasks: Vec::new() };

    database.add(task1);
    database.add(task2);
    database.add(task3);

    database.list();
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

#[derive(Debug)]
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
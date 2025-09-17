use std::{env, process};
use to_do_app::{Config, Database, Task};

fn main() {
    let file_path = "tasks.json";

    let mut database: Database = Database::load(file_path).unwrap_or_else(|err| {
        eprintln!("Error loading the file 'tasks.json': {err}");
        process::exit(1);
    });

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing the commands: {err}");
        process::exit(1);
    });

    let task: Task = Task::build(config).unwrap_or_else(|err| {
        eprintln!("Error creating the task: {err}");
        process::exit(1);
    });

    database.add(task);
    database.list();

    if let Err(err) = database.save(file_path) {
        eprintln!("Error saving the file: {err}");
        process::exit(1);
    };
}

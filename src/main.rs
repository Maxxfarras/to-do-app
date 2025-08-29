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

    let mut database: Vec<Task> = Vec::new();

    database.push(task1);

    println!("{:#?}", database);


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
            None => return Err("No command parsed!!")
        };

        let argument = args.next();

        Ok(Self { command, argument})
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

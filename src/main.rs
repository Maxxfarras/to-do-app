use std::{env, process};

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing the commands: {err}");
        process::exit(1);
    });

    println!("{:#?}", config);
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

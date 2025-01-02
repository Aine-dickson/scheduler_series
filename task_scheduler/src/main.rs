use std::{fs::{exists, read_to_string, File}, process::exit};

struct Task {
    priority: u8,
    length: usize
}

fn main() {
    let exists_tasks = exists("tasks.json").unwrap();

    if !exists_tasks {
        let creator = File::create("tasks.json");
        match creator {
            Ok(file) => {}
            Err(er) => {
                println!("Error: {}\nExiting due to failure to create tasks file", er);
                exit(1);
            }
        }
    }

    let tasks_result = read_to_string("tasks.json");
    let mut tasks: Vec<String> = vec![];

    match tasks_result {
        Ok(value) => tasks = value.split_whitespace().map(|s| s.to_string()).collect(),
        Err(_) => {
            println!("Error while opening tasks file\nExting...");
            exit(1);
        },
    }

    println!("{:#?}", tasks);
}

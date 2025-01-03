use std::{env::args, fs::{exists, read_to_string, File, OpenOptions}, io::{stdin, Write}, process::{exit, Command, Stdio}};

use sysinfo::{Pid, System};
use task_lib::Task;

fn main() {

    let mut args_ = args().skip(1).peekable();

    if !exists("../tasks.json").unwrap() {
        let creator = File::create("../tasks.json");
        match creator {
            Ok(mut file) => {
                file.write_all("[]".as_bytes()).unwrap();
                file.flush().unwrap();
            }
            Err(er) => {
                println!("Error: {}\nExiting due to failure to create tasks file", er);
                exit(1);
            }
        }
    }

    let mut tasks: Vec<Task> = vec![];

    match read_to_string("../tasks.json") {
        Ok(value) => {
            match serde_json::from_str(&value) {
                Ok(_tasks) => tasks = _tasks,
                Err(err) => {
                    println!("Failed to parse tasks from tasks file\nExiting...{}", err);
                    exit(1);
                },
            }
        }
        Err(_) => {
            println!("Error while reading tasks file\nExiting...");
            exit(1);
        },
    }

    while let Some(arg) = args_.peek() {
        match arg.as_str() {
            "list" => {
                println!("You need a list?")
            }
            "run" => {
                run_engine_process();
            }
            "stop" => {
                stop_engine();
            }
            "help" => {
                println!("You need help?")
            }
            "add" => {
                create_task(&mut tasks)
            }
            "modify" => {
                println!("You need help?")
            }
            "delete" => {
                println!("You need help?")
            }
            _ => {
                println!("Unknown argument {}", arg)
            }
        }

        args_.next();
    }


}

fn run_engine_process() {
    if let Ok(child_p) = Command::new("cargo").arg("run").current_dir("../engine").stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn() {
        if !exists("pid.txt").unwrap() {
            File::create("pid.txt").unwrap();
        }

        if let Ok(mut pid_file) = OpenOptions::new().write(true).truncate(true).open("pid.txt") {
            pid_file.write_all(format!("{}", child_p.id()).as_bytes()).unwrap();
            pid_file.flush().unwrap();
        }

    }
}

fn stop_engine(){
    let pid = read_to_string("pid.txt").unwrap();
    let mut system = System::new_all();
    system.refresh_all();
    if let Some(process) = system.process(Pid::from(pid.parse::<usize>().unwrap())) {
        process.kill();
    }
}

fn create_task(tasks: &mut Vec<Task>) {
    let mut new_task = Task::new(6, 0, String::new());

    for count in 1..=4 {
        match count {
            1 => {
                println!("Provide task priority in range of 1 to 6");
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        new_task.modify(Some(input.trim().parse().unwrap()), None, None);
                    },
                    Err(_) => {
                        println!("Error while reading input\nTry again");
                        exit(1);
                    },
                }
            }
            2 => {
                println!("Provide time in ms that task requires to finish execution");
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        new_task.modify(None, None, Some(input.trim().parse::<usize>().unwrap()));
                    },
                    Err(_) => {
                        println!("Error while reading input\nTry again");
                        exit(1);
                    },
                }
            }
            3 => {
                println!("Provide task name");
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        new_task.modify(None, Some(input.trim().to_string()), None);
                    },
                    Err(_) => {
                        println!("Error while reading input\nTry again");
                        exit(1);
                    },
                }
            }
            _ => {}
        }
    }

    tasks.push(new_task);
    let tasks_json_fmt = serde_json::to_string_pretty(tasks).unwrap();

    let mut file = OpenOptions::new().truncate(true).write(true).open("../tasks.json").unwrap();
    file.write_all(tasks_json_fmt.as_bytes()).unwrap();
    file.flush().unwrap();
}
use std::{fmt::Display, fs::read_to_string};

use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: String,
    priority: u8,
    name: String,
    length: usize,
    state: TaskStatus,
    elapsed_length: usize
}

#[derive(Debug, Deserialize, Serialize)]
enum TaskStatus {
    Halted,
    Running,
    Pending,
    Finished,
    Failed(String)
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "> {}:\n   -Status: {}\n   -Size left: {}", self.name, self.state, self.length)
    }
}
impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Running => write!(f, "Pending"),
            TaskStatus::Halted => write!(f, "Halted"),
            TaskStatus::Finished => write!(f, "Finished"),
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Failed(reason) => write!(f, "Failed: {}", reason)
        }
    }
}

impl Clone for TaskStatus {
    fn clone(&self) -> Self {
        match self {
            Self::Halted => Self::Halted,
            Self::Running => Self::Running,
            Self::Pending => Self::Pending,
            Self::Finished => Self::Finished,
            Self::Failed(arg0) => Self::Failed(arg0.clone()),
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.length == other.length && self.elapsed_length == other.elapsed_length
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority).then(self.length.cmp(&other.length).then(self.elapsed_length.cmp(&other.elapsed_length))))
    }
}

impl Task {
    pub fn new(priority:u8, length:usize, name:String)-> Self where Self: Sized {
        Self {
            priority,
            length,
            name,
            id: randomizer(),
            elapsed_length: 0,
            state: TaskStatus::Pending,
        }
    }

    pub fn modify(&mut self, priority:Option<u8>, name:Option<String>, length:Option<usize>) {
        if priority.is_some() {
            self.priority = priority.unwrap();
        }

        if name.is_some() {
            self.name = name.unwrap();
        }

        if length.is_some() {
            self.length = length.unwrap()
        }
    }

    pub fn modify_status(&mut self, new: TaskStatus)-> Result<(), String> {
        match (self.state.clone(), &new) {
            (TaskStatus::Halted, TaskStatus::Running) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Halted, TaskStatus::Finished) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Running, TaskStatus::Halted) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Running, TaskStatus::Running) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Running, TaskStatus::Finished) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Pending, TaskStatus::Running) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Pending, TaskStatus::Finished) => {
                self.state = new;
                Ok(())
            },
            (TaskStatus::Failed(_), TaskStatus::Pending) => {
                self.state = new;
                Ok(())
            }
            
            // Not allowed transissions
            (_, TaskStatus::Halted) => {
                Err("Cannot transition to halted status unless it's a running task".to_string())
            },
            (_, TaskStatus::Pending) => {
                Err("Cannot transition to pending status unless it's a failed task".to_string())
            },
            (TaskStatus::Finished, _) => {
                let err = format!("Task {} is already complete. Cannot be transitioned", self.name);
                Err(err)
            },
            (_, TaskStatus::Failed(_)) => Err("Cannot transition failed status".to_string()),
            (TaskStatus::Failed(_), _) => Err("Transition from failed status is only allowed to pending status".to_string())
        }
    }
}

fn randomizer()-> String {
    let length = rand::thread_rng().gen_range(10..=20);
    rand::thread_rng().sample_iter(Alphanumeric).take(length).map(char::from).collect()
}

pub fn retrieve_tasks()-> Vec<Task> {
    if let Ok(data) = read_to_string("../tasks.json") {
        if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&data) {
            return tasks;
        }
    }

    Vec::new()
}
use std::fs::read_to_string;
use task_lib::Task;

pub struct Scheduler {
    pub tasks: Vec<Task>,
    next_task: Option<String>,
    running_task: Option<String>,
}

impl Scheduler {
    pub fn new()->Self {
        Self{
            tasks: fetch_tasks(),
            running_task: None,
            next_task: None
        }
    }

    pub fn run(&mut self) {
        // task_container_watcher
    }
}

fn fetch_tasks()-> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];

    if let Ok(data) = read_to_string("tasks.json") {
        tasks = serde_json::from_str(&data).unwrap();
    }

    tasks
}
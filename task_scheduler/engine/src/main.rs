use std::{cell::RefCell, fs::{metadata, read_to_string}, ops::Deref, rc::Rc, sync::{mpsc::channel, Arc, Mutex}, thread::{self, sleep}, time::{Duration, SystemTime}};

use scheduler::Scheduler;
use task_lib::{retrieve_tasks, Task};

mod scheduler;

fn main() {
    let (sender, receiver) = channel();
    
    let file_watch_thread = thread::spawn(move ||{
        let mut access_modification_time: SystemTime = SystemTime::now();
        if let Ok(meta) = metadata("../tasks.json") {
            if let Ok(sys_time) = meta.modified() {
                access_modification_time = sys_time;
            }
        }

        loop {
            if let Ok(meta) = metadata("../tasks.json") {
                if let Ok(sys_time) = meta.modified() {
                    if sys_time.gt(&access_modification_time) {
                        sender.send(retrieve_tasks()).unwrap();
                    }
                }
            }
        }
    });

    let scheduler_atomic_ref = Arc::new(Mutex::new(Scheduler::new()));
    let scheduler_atomic_ref_clone = Arc::clone(&scheduler_atomic_ref);

    if let Ok(mut scheduler) = scheduler_atomic_ref.lock() {
        scheduler.tasks = retrieve_tasks();
    }

    let tasks_syncronizer_thread = thread::spawn(move ||{
        for received in receiver {
            let tasks_iter = received.into_iter();
            
            for task in tasks_iter {
                if let Ok(mut scheduler) = scheduler_atomic_ref_clone.lock() {
                    if !scheduler.tasks.contains(&task) {
                        scheduler.tasks.push(task);
                        println!("{:#?}", scheduler.tasks)
                    }
                }
            }
        }
    });

    // if let Ok(scheduler) = scheduler_atomic_ref.lock() {
    //     loop {
    //         println!("Tasks length: {}", scheduler.tasks.len());
    //         sleep(Duration::from_micros(100));
    //     }
    // }

    file_watch_thread.join().unwrap();
}


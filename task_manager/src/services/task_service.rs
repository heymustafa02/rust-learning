use crate::models::task::Task;
use crate::storage::file_storage;

pub struct TaskService {
    pub tasks: Vec<Task>,
}

impl TaskService {
    pub fn new() -> Self {
        TaskService {
            tasks: file_storage::load().unwrap_or_default(),
        }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
        file_storage::save(&self.tasks).unwrap();
    }

    pub fn mark_done(&mut self, id: u32) {
        for t in &mut self.tasks {
            if t.id == id {
                t.done = true;
            }
        }
        file_storage::save(&self.tasks).unwrap();
    }

    pub fn list(&self) {
        for t in &self.tasks {
            println!(
                "{}. [{}] {}",
                t.id,
                if t.done { "x" } else { " " },
                t.title
            );
        }
    }

    pub fn filter_done(&self) {
        for t in &self.tasks {
            if t.done {
                println!("{}. [x] {}", t.id, t.title);
            }
        }
    }
}

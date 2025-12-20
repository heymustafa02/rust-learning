use serde::{Serialize, Deserialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    task: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, task: String) {
        let id = self.todos.len() + 1;
        let todo = Todo { id, task, done: false };
        self.todos.push(todo);
        println!("Todo added.");
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("No todos yet.");
            return;
        }

        for todo in &self.todos {
            println!(
                "{}. [{}] {}",
                todo.id,
                if todo.done { "x" } else { " " },
                todo.task
            );
        }
    }

    fn mark_done(&mut self, id: usize) {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.done = true;
                println!("Todo marked as done!");
                return;
            }
        }
        println!("Todo not found.");
    }

    fn remove(&mut self, id: usize) {
        self.todos.retain(|todo| todo.id != id);
        println!("Todo removed.");
    }

    fn save(&self) -> Result<(), io::Error> {
        let data = serde_json::to_string_pretty(&self).unwrap();
        fs::write("todos.json", data)?;
        Ok(())
    }

    fn load() -> Self {
        let data = fs::read_to_string("todos.json").unwrap_or("".to_string());

        if data.trim().is_empty() {
            return TodoList::new();
        }

        serde_json::from_str(&data).unwrap_or(TodoList::new())
    }
}

fn main() {
    let mut todo_list = TodoList::load();

    println!("Todo CLI");
    println!("Commands:");
    println!("1 add <task>");
    println!("2 list");
    println!("3 done <id>");
    println!("4 remove <id>");
    println!("5 exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                let task = parts[1..].join(" ");
                todo_list.add(task);
                let _ = todo_list.save();
            }

            "list" => {
                todo_list.list();
            }

            "done" => {
                if parts.len() < 2 {
                    println!("Usage: done <id>");
                    continue;
                }
                let id: usize = parts[1].parse().unwrap_or(0);
                todo_list.mark_done(id);
                let _ = todo_list.save();
            }

            "remove" => {
                if parts.len() < 2 {
                    println!("Usage: remove <id>");
                    continue;
                }
                let id: usize = parts[1].parse().unwrap_or(0);
                todo_list.remove(id);
                let _ = todo_list.save();
            }

            "exit" => {
                println!("Saving and exiting...");
                let _ = todo_list.save();
                break;
            }

            _ => println!("Unknown command."),
        }
    }
}

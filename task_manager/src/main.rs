use task_manager::models::task::Task;
use task_manager::services::task_service::TaskService;
use task_manager::utils::id::generate_id;

use std::io;

fn main() {
    let mut service = TaskService::new();

    println!("Task Manager");
    println!("Commands: add <title> | list | done <id> | done_only | exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                let title = parts[1..].join(" ");
                let id = generate_id();
                service.add(Task::new(id, title));
            }

            "list" => service.list(),

            "done" => {
                if parts.len() < 2 {
                    println!("Usage: done <id>");
                    continue;
                }
                let id: u32 = parts[1].parse().unwrap_or(0);
                service.mark_done(id);
            }

            "done_only" => service.filter_done(),

            "exit" => {
                println!("Bye!");
                break;
            }

            _ => println!("Unknown command."),
        }
    }
}

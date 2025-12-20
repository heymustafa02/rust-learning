mod db;          // import database module
mod commands;    // import command handler module

use std::io;
use crate::db::Database;
use crate::commands::handle_command;

fn main() {
    let mut db = Database::new();

    println!("Simple KV Store");
    println!("Commands:");
    println!("set <key> <value>");
    println!("get <key>");
    println!("delete <key>");
    println!("list");
    println!("exists <key>");
    println!("count");
    println!("clear");
    println!("keys");
    println!("exit");
    println!("----------------------------------");

    loop {
        let mut input = String::new();

        // Read input from stdin
        io::stdin().read_line(&mut input).unwrap();

        // Split input into words
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        // Handle the command using our commands module
        handle_command(&mut db, parts);
    }
}

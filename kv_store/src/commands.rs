use crate::db::Database;

pub fn handle_command(db: &mut Database, parts: Vec<&str>) {
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        // set key value
        "set" => {
            if parts.len() < 3 {
                println!("Usage: set <key> <value>");
                return;
            }

            let key = parts[1].to_string();
            let value = parts[2..].join(" "); // support multi-word values

            db.set(key, value);
            println!("OK");
        }

        // get key
        "get" => {
            if parts.len() < 2 {
                println!("Usage: get <key>");
                return;
            }

            match db.get(parts[1]) {
                Some(v) => println!("{}", v),
                None => println!("(nil)"),
            }
        }

        // delete key
        "delete" => {
            if parts.len() < 2 {
                println!("Usage: delete <key>");
                return;
            }

            if db.delete(parts[1]) {
                println!("Deleted");
            } else {
                println!("Key not found");
            }
        }

        // list keys and values
        "list" => db.list(),

        // exists key
        "exists" => {
            if db.exists(parts[1]) {
                println!("true");
            } else {
                println!("false");
            }
        }

        // count items
        "count" => println!("{}", db.count()),

        // clear database
        "clear" => {
            db.clear();
            println!("All keys cleared.");
        }

        // show all keys
        "keys" => db.keys(),

        // exit
        "exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }

        // unknown command
        _ => println!("Unknown command"),
    }
}

// Basic Serde Example in Rust
use serde::{ Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

struct User {
    username: String,
    password: String
}

fn main () {
    let u: User = User {
        username: String::from("Mustafa"),
        password: String::from("12345")
    };
    let serialized_string: Result<String, serde_json::Error> = serde_json::to_string(&u);
    match serialized_string {
        Ok(s) => println!("Serialized User: {}", s),
        Err(_) => println!("Serialization Error"),
    }
}

// Serde Yaml Example in Rust
use serde::{Deserialize as DeserializeSerde, Serialize as SerializeSerde};

#[derive(SerializeSerde, DeserializeSerde, Debug, PartialEq)]
struct User {
    username: String,
    password: String
}

#[derive(Debug, PartialEq, SerializeSerde, DeserializeSerde)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>,
    user: User
}

fn main() {
    let s = MyStruct {
        id: 32,
        data: String::from("Mustafa"),
        v: vec![1, 2, 3],
        user: User {
            username: String::from("Mustafa"),
            password: String::from("Mallebhari")
        }
    };
    let json_str = serde_yaml::to_string(&s).unwrap();
    print!("{}", json_str);
    let deserialized: MyStruct = serde_yaml::from_str(&json_str).unwrap();
    assert_eq!(deserialized, s);
    println!("done!")    
}


// Converting string to binary and back to string
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]

struct User {
    username: String,
    password: String
}

fn main() {
   let u: User = User {
        username: String::from("Mustafa"),
        password: String::from("12345")
    };
    let mut v: Vec<u8> = Vec::new();

    let ans = u.serialize(&mut v);

    println!("Serialized User: {:?}", v);
    let user= User::try_from_slice(&v).unwrap();
    println!("Deserialized User: {:?}", user.username);
   }

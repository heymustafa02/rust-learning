// fn main() {
//     let name = "Mustafa"; // immutable
//     let mut score = 0; // mutable

//     println!("Name: {}", name);
//     println!("Initial score: {}", score);

//     score = 50;
//     println!("Updated score: {}", score);

//     const PI: f64 = 3.14;
//     println!("Pi: {}", PI);
// }
//borrowing example
// fn main() {
//     let a = String::from("hello");
//     let b = &a; // borrow a, NOT move

//     println!("a: {}", a); // works
//     println!("b: {}", b); // works
// }



// Mutable borrowing example
// fn main() {
//     let mut name = String::from("Mustafa");

//     let n = &mut name;  // mutable borrow
//     n.push_str(" Rustacean");

//     println!("{}", n);
// }
// another borrowing example
// fn main() {
//     let mut language = String::from("Rust");

//     let l1 = &language;
//     println!("l1: {}", l1);

//     // l1 is no longer used after this line → borrow ends here

//     let l2 = &mut language;
//     l2.push_str(" programming");
//     println!("l2: {}", l2);
// }


//function with parameter basic
// fn greet(name: &str, rollno: u32) {
//     println!("Hello, name- {} rollno -{}", name, rollno);
// }

// fn main() {
//     greet("Mustafa", 42);
// }

// structs, impl, enums

// struct Student {
//     name: String,
//     marks: u32,
//     remarks: String,
// }

// impl Student {
//     fn greet(&self) {
//         println!("Result of {}", self.name);
//     }

//     fn add_marks(&mut self, extra: u32) {
//         self.marks += extra;
//     }
//     fn remarks(&self) {
//         println!("Remarks: {}", self.remarks);
//     }
// }

// enum Status {
//     Pass,
//     Fail,
// }

// fn main() {
//     let mut s = Student {
//         name: String::from("Mustafa"),
//         marks: 0,
//         remarks: String::from("Not so good"),
//     };

//     s.greet();
//     s.add_marks(20);
//     s.remarks();

//     let status = if s.marks >= 50 {
//         Status::Pass
//     } else {
//         Status::Fail
//     };

//     match status {
//         Status::Pass => println!("You passed!"),
//         Status::Fail => println!("You failed!"),
//     }
// }



// fn main() {
//     let n=23;
//     let x = 225;
//     // match x {
//     //     0..12 => println!("child"),
//     //     13..19 => println!("teenager"),
//     //     _=> println!("Adult"),
//     // }
//         match n {
//         1 | 2 | 3 => println!("Small number"),
//         _ => println!("Other"),
//         }
//     // println!("Value of x is: {}", x);
   
// }
// fn main() {
//     let point = (45, 45);

//     match point {
//         (0, y) => println!("On x-axis at y = {}", y),
//         (x, 0) => println!("On y-axis at x = {}", x),
//         (x, y) => println!("Point at ({}, {})", x, y),
//     }
// }

// matching
// fn safe_div(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err("Division by zero".into())
//     } else {
//         Ok(a / b)
//     }
// }

// fn main() {
//     match safe_div(120, 5) {
//         Ok(v) => println!("Result = {}", v),
//         Err(e) => println!("Error: {}", e),
//     }
// }

//vectors
// fn main() {
//     let mut numbers: Vec<i32> = Vec::new();

//     numbers.push(10);
//     numbers.push(20);
//     numbers.push(30);

//     println!("{:?}", numbers);
// }


// fn main() {
//     let nums = vec![1, 2, 3, 4];
//     println!("{:?}", nums);
// }


// fn main() {
//     let mut v = vec![10, 20, 30];

//     println!("First = {}", v[0]);

//     v[1] = 200;
//     println!("{:?}", v);
// }

//push, pop, insert, remove

// fn main() {
//     let mut v = vec![1, 2, 3];

//     v.push(4);
//     println!("After push: {:?}", v);

//     v.pop();
//     println!("After pop: {:?}", v);

//     v.insert(1, 100);
//     println!("After insert: {:?}", v);

//     v.remove(0);
//     println!("After remove: {:?}", v);
// }


//HashMaps
// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert("Mustafa", 90);
//     scores.insert("Aman", 85);
//     scores.insert("Sara", 95);

//     println!("{:?}", scores);
// }


//Traits example


// Example of {}, {:? }, and {:#? } formatting

// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
// }

// use std::fmt;

// impl fmt::Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} ({}) years old", self.name, self.age)
//     }
// }

// fn main() {
//     let u = User {
//         name: "Mustafa".into(),
//         age: 19,
//     };

//     // Debug print
//     println!("{:?}", u);

//     // Pretty debug
//     println!("{:#?}", u);

//     // Display print (requires custom impl)
//     println!("{}", u);
// }

// Lifetimes example
// fn max_str<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         println!("{} is greater", a);
//         a
//     } else {
//         println!("{} is greater", b);
//         b
//     }
// }

// fn main() {
//     let x = "Mustafa";
//     let y = "Mallebhari";

//     let m = max_str(x, y);
//     // println!("Max string: {}", m);
// }

fn longest(a: &String, b: &String) -> &String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("longer");

    let res = longest(&s1, &s2);
    println!("{}", res);
}

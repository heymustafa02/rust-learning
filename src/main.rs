// fn main () {
//     println!("{}", is_even(-88));

// } 
// fn is_even(num:i64)-> bool {
//     if num % 2==0 {
//         return true;
//     }
//     return false;
//     }
 
// fibbonaci series
// fn main() {
//     let n = 10; // Change this value to generate more terms
//     let mut a = 0;
//     let mut b = 1;
//     let mut fib_sequence = vec![a, b];
//     for _ in 2..n {
//         let next = a + b;
//         fib_sequence.push(next);
//         a = b;
//         b = next;
//     }
//     println!("Fibonacci sequence up to {} terms: {:?}", n, fib_sequence);
// }

// Fibonacci series
// fn main(){
// 	println!("{}", fib(7))
// }

// fn fib(num:u32) -> u32 {
// 	let mut frst =0;
// 	let mut sec = 1;

// 	if num ==0 {
// 	return frst;
//     }
//     if num == 1 {
//     return sec;
//     }

//     for _ in 0..(num -1 ){
//         let temp =sec;
//         sec = frst + sec;
//         frst = temp;
//     }
//     return sec;
// }

// write function get_string_lenght that takes a string as an input and returns its length

// fn get_string_length(s: &str) -> usize {
//     s.chars().count()
// }   
// fn main() {
//     let my_string = "Hello, world!";
//     let length = get_string_length(my_string);
//     println!("The length of the string is: {}", length);
// }


struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main (){
    let user = User {
        first_name: String::from("Mustafa"),
        last_name: String::from("Mallebhari"),
        age: 20,
    };

    println!("User Details:");
    println!("First Name: {}", user.first_name);
    println!("Last Name: {}", user.last_name);
    println!("Age: {}", user.age);
    println!("Full Name and Age: {} {} {}", user.first_name, user.last_name, user.age);
}


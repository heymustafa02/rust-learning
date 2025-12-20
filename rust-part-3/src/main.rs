// use chrono::prelude::*;
// use dotenv::dotenv;
// use std::env;
use std::ops::Add;
fn main() {

    // println!("Hello, world!");
    // UTC and Local time example
    // let utc: DateTime<Utc> = Utc::now();
    // let local: DateTime<Local> = Local::now();
    // println!("Current UTC time: {}", utc);
    // println!("Current Local time: {}", local);

    // Load environment variables from .env file
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("Database URL: {}", database_url);
   
   // Generics and Traits example
   println!("Sum of integers: {}", sum(10, 20));

   fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
       return a + b
   }
}
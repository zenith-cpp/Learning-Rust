use crate::ansi_color_codes;
use crate::essentials;

pub fn conditions(age: i32) {
    if age >= 18 {
        println!("Technically an adult [{}]", age);
    } else if age <= 18 {
        println!("Not an adult [{}]", age);
    }
    
}

pub fn short_condition(age: i32) {
    essentials::clear();
    let status = if age >= 18 {"Adult"} else {"Minor"};
    println!("[{}{}let status = if age >= 18 {{'Adult'}} else {{'Minor'}}{}] Status: {}", ansi_color_codes::BOLD, ansi_color_codes::GREEN, ansi_color_codes::RESET, status);
}
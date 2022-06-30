// Ch 3 Control Flow
/* 
fn main() {
    let number = 7;

    if number < 5{
        println!("condition was true"); // condition must be a bool or you get an error
    } else {
        println!("condition was false");
    }
} */
/*
fn main() { // else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
} */
/* 
// if in a let statement
fn main() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");
} */

fn main() {
    loop {
        println!("again");
    }
}
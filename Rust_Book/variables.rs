// Ch 3
// Variables & Mutability
/* 
fn main() {
    let x = 5;
    println!("The value of x is: {}", x); // Does not compile.  X is non mutable but assigned to two different values
    x = 6;
    println!("The value of x is: {}", x);
} */
/* 
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
*/
// The value of x is: 5
// The value of x is: 6

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is; {}", x);
}
// The value of x in the inner scope is: 8
// The value of x is; 6
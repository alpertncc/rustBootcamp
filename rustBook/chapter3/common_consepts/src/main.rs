// 3.1. Variables and Mutability

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");    

    // After removing `mut` keywork in front of the `x` variable, you will recieve an error message "cannot assign twice to immutable variable `x` because `x` is immutable".
    // To make the `x` mutable, you HAVE TO add `mut` keyword in front of the `x`.
}
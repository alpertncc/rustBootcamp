// 3.1. Variables and Mutability

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");    

    // You will recieve an error message "cannot assign twice to immutable variable `x` because `x` is immutable "

}
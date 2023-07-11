// 3.1. Variables and Mutability

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");    

    // After removing `mut` keywork in front of the `x` variable, you will recieve an error message "cannot assign twice to immutable variable `x` because `x` is immutable".
    // To make the `x` mutable, you HAVE TO add `mut` keyword in front of the `x`.
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of c in the innerscope is: {x}");
    }

    println!("The wvalue of x is{x}");

    // The output of the `shadowing` function will be;
    // $ The value of x in the inner scope is: 12
    // $ The value of x is: 6
}


// DATA TYPES

fn data_types() {

    let x: u64 = 1548645132; // u64

    let y = 2.0; // f64

    let z: f32 = 3.2; //f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; //with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

}

fn tupples() {

    let tup: (i32, f64, u8) = (500, 6.4, 1); 

    let (x, y, z) = tup;

    println!("The value of y is: {y}"); // 6.4

    // This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called **destructuring** because it breaks the single tuple into three parts.

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

}

fn arrays() {

    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];
    
    {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. 
    //You can access elements of an array using indexing, like this.
    }


}
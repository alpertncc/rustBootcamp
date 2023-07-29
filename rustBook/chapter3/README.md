# Rust Notes on Rust Book Chapter 3

- Open ***common_consepts/src/main.rs*** file and run the ``` $ cargo run ``` command to see the errors, outputs and code lines.

<details>

<summary>Variables and Mutability</summary>

## 3.1. Variables and Mutability

- By default, *variables* in Rust are immutable. So that Rust gives you to write your code in a way that takes advantage of the saftey and easy concurrency that Rust offers. 

- When a variable is immutable, you **CAN NOT** change that value.

```rust
 let x = 5; 
 x = 6; 
 ```
 This code block will cause an error. You have to use ``` mut ``` keyword to make a variable mutable. 

 ```rust
 let mut x = 5; 
 x = 6; 
 ```
### Constants 

- *Constants* are also values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables. 

1. You ***ARE NOT*** allowed to use ``` mut ``` keyword with *constants*. Constants are just immutable by default. 

1. You declare constants using the ``` const ``` keyword instead of the ``` let ``` keyword, and the type ***MUST BE*** annotated.

1. Constants can be declared in any scope, including the global scope, which makes them useful for values that may parts of code need to know about.

1. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime. 

- An example of a constant declaration: 

```rust
conts THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

-Rust's naming convention for constants is to use all uppercase with underscores between words. 

## Shadowing

- To see the example of shadowing, look at the *main.rs* file. 

- Shadowing is different from marking a variable as ``` mut ```. By using ```let```, we can perform a few transformation on a value but have the variable be immutable after transformations have been completed.

- The other difference between ``` mut ``` and *shadowing* is that because we're effectively creating a new variable when we use the ``` let ``` keyword again, we can change the type of the value but reuse the same name.

</details>

<details>

<summary>Data Types</summary>

## 3.2. Data Types

- We will look at two data type subsets: ***scalar*** and ***compound***.

- Rust is a *statically typed language*, which means that it must know the types of all variables at compile time.

1. Scalar Types

Rust has four primary scalar types: ***integers, floating-point numbers, Booleans and characters***.

- Integer Types

| Length | Signed | Unsigned |  
|-----------|:-----------:|-----------:|  
| 8-bit | i8 | u8 |  
| 16-bit | i16 | u16 | 
| 32-bit | i32 | u32 | 
| 64-bit | i64 | u64 | 
| 128-bit | i128 | u128 | 
| arch | isize | usize | 

*Signed and Unsigned* refer to whether it's possible for the number to be negative, in other words, whether the number need s to have a sign with it (*signed*) or whether it will be only ever be positive and cam therefore be represented without a sign (*unsigned*).


- Floating-Point Types

Rust's floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectivelt. 

- The Boolean Type

a Boolean type in Rust has two possible values, `true` and `false`. 

Booleans are ***ONE BYTE*** in size. 

The Bloolean Type in Rust is specified using ``` bool ```

- The Character Type

Rust's ``` char ``` type is the language's most primitive alphabetic type. To find the example, look at the ***common_cpnsepts/src/main.rs*** file. 

We specify ``` char ``` literals with *single quotes*, as opposed to strinf literals, which use double quotes. 

Rust's ``` char ``` type is **four bytes** şn size and represents a **Unicode Scalar Value**, which means it can represent a lor more than just ***ASCII***.

1. Compound Types 

Compound types can group multiple values into one type. Rust has two primitive compound types: ***tuples*** and ***arrays***.

- The Tuple Type 

A tuple is a general way of grouping together a number of values *with a variety of types* into one compound type. 

*Tuples have a fixed length: once declared, they cannot grow or shrink in size.*

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

- The Array Type

Unlike a tuple, every element of an array *must have the same type*. 

Unlike arrays in some other languages, arrays in Rust have a fixed length.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements. 

An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
//Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
```

```rust
 let a = [3; 5]; // let a = [3, 3, 3, 3, 3];
 ```

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

// An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. 
//You can access elements of an array using indexing, like this.
}
```

</details>

<details> 

<summary> Functions </summary>

- We define a function in Rust by entering fn followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.

```rust
fn main() {
    println!("Hello, world!");

    another_function(); 
    // We can call any function we’ve defined by entering its name followed by a set of parentheses.
}

fn another_function() {
    println!("Another function.");
}
```
- we defined another_function after the main function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

## Parameters

- We can define functions to have parameters, which are special variables that are part of a function’s signature.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// The output is "The value of x is: 5"
```

- In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.

- When defining multiple parameters, separate the parameter declarations with commas.

## Statements and Expressions

- ***Statements*** are instructions that perform some action and do not return a value. 
- ***Expressions*** evaluate to a resultant value.

## Functions and Return Values

- Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (***->***).

- You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
// The output will be "The value of x is: 5".
```

</details>

<details> 

<summary> Comments </summary>

- In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line. For comments that extend beyond a single line, you’ll need to include ***//*** on each line

```rust
fn main() {
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
}
```

</details>



<details> 

<summary> Control Flow </summary>

- The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

## If Expressions

- An if expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

### Handling Multiple Conditions with ***else if***

- You can use multiple conditions by combining if and else in an else if expression. For example:

```rust
fn main() {
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
}
// The output is "number is divisible by 3".
```

- Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. 

### Using ***if*** in a ***let*** statement

- Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
/// The output is "The value of number is: 5".
```

## Repetition with Loops

- It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning. 

- Rust provides a way to break out of a loop using code. You can place the ***break*** keyword within the loop to tell the program when to stop executing the loop.

- Rust has three kinds of loops: ***loop***, ***while***, and ***for***. 

1. Repeating Code with ***loop***

- The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
/* The output is;
again!
again!
again!
again!
...
*/
```

1. Returning Values from Loops

- One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code.
- To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

1. Conditional Loops with ***while***

- A program will often need to evaluate a condition within a loop. 

- While the *condition is true*, the loop runs. 
- When the *condition ceases to be true*, the program calls break, stopping the loop. 

- It’s possible to implement behavior like this using a combination of loop, if, else, and break; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a ***while loop***.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
## Looping Through a Collection with ***for***

- The safety and conciseness of for loops make them the most commonly used loop construct in Rust.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Using the for loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array.

</details>
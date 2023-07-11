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
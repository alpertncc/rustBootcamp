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

- The other difference between ```mut ``` and *shadowing* is that because we're effectively creating a new variable when we use the ``` let ``` keyword again, we can change the type of the value but reuse the same name.

</details>

<details>

<summary>Data Types</summary>

## 3.2. Data Types

</details>
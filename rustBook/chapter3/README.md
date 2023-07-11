# Rust Notes on Rust Book Chapter 3

<details>

<summary>Variables and Mutability</summary>

### 3.1. Variables and Mutability

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
 

</details>

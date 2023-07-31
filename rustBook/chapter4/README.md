# Chapter 4. Understanding Ownership

- Ownership, References and Borrowing are important topics on Rust. We must learn these topics as best we can. So, please follow the instructions and read the notes carefully.

<details> 

<sumamry> 4.1 What is Ownership? </summary>

- To see the codes for this part of the Chapter 4, open the ```ownership/src/main.rs``` file. To run the code, run the ```$cargo run``` command in the ***ownership*** folder. 

Ownership is a set of rules that govern how a Rust program manages memory.

All programs have to manage the way they use a computer’s memory while running. 

- Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs. 

- In other languages, the programmer must explicitly allocate and free the memory. 

- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

## The Stack and the Heap

- Both ***the stack*** and ***the heap*** are parts of memıry available to your code to use at runtime.

-  ***The Stack*** stores values in the order it gets them and removes the values in the opposit order; ***LAST IN, FIRST OUT*** method. When you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding data is called ***pushing onto the stack***, and removing data is called ***popping off the stack***. All data stored on the stack ***must have a known, fixed size***. 

- ***The Heap*** is less organized, when you put data on the heap, you request a certain amount of space. The memory allocator marks the empty spot in the heap as being in use, and returns ***a pointer***, which is the address of that location. This process is called ***allocating on the heap***.

- Pushing to the stack ***is faster*** than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap ***requires more work*** because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 

### Ownership Rules
- First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

1. Each value in Rust has an owner.
1. There can only be one owner at a time.
1. When the owner goes out of scope, the value will be dropped.

### Variable Scope 

- A scope is the range within a program for which an item is valid.

### The String Type

We’ve already seen string literals, where a string value is hardcoded into our program. String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. 

- One reason is that they’re immutable. 

- Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? 

For these situations, Rust has a second string type, ***String***. This type manages data allocated **on the heap** and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:

```rust
let s = String::from("hello");
```

- The ***double colon ::*** operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.

### IMPORTANT

- To continue to my notes, please open the `chapter4/ownership/src/main.rs` file and examine the code. I also suggest you to read the notes!

- To understand better, please go to the [relevant section of the Rust Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation) and read these section;
1. Memory and Allocation
1. Variables and Data Interacting with Clone
1. Stack-Only Data: Copy
1. Ownership and Functions
1. Return Values and Scope

</details>

<details>

<summary> References and Borrowing </summary>

-  A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

- Note: The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`.

- When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

- We call the action of creating a reference ***borrowing***.

- To undertand better, I suggest you to examine the code blocks in the `chapter4/references_borrowing/src/main.rs` file.
- Also, please go to the [relevant section of the Rust Book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing) and read the relevant page. 

</details>

<details>

<summary> The Slice Type </summary>

- ***Slices*** let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

- To understand better, you MUST read the [relevant section from the Rust Book](https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type). 

- I did not prepare a code file for the slices because while reading the link above, you MUST try the code blocks on your own. This way is better for you to learn slices.


</details>



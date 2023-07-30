# Chapter 4. Understanding Ownership

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


</details>

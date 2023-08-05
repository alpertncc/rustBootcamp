### Using Structs to Structure Related Data

- A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

<details> 

<summary> Defining and Instantiating Structs </summary>

- To examine the usage of ***Structs***, check the file, `chapter5/structs/src/main.rs`.
- Structs are similar to tuples. The pieces of a struct can be different types. 

- In a struct you’ll name each piece of data so it’s clear what the values mean.

- To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together.

- Inside curly brackets, we define the names and types of the pieces of data, which we call ***fields***.

- To use a struct after we’ve defined it, we create an ***instance*** of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing ***key: value*** pairs, where the keys are the names of the fields and the values are the data we want to store in those fields.

## Unit-Like Structs Without Any Fields

- Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 

- ***Traits*** is the topic of Chapter 10. 

## Ownership of Struct Data

- In the User struct which is in the `chapter5/structs/src/main.rs` file, we used the owned String type rather than the &str string slice type. This is a *deliberate choice* because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else, but to do so requires the use of ***lifetimes***, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.


</details>

<details> 

<summary> 5.2. Examples for Structs </summary>

You can follow the notes from the `chapter5/exampleProgram/src/main.rs` file untill the `dbg!` macro.

## dbg! Macro

- Another way to print out a value using the Debug format is to use the ***dbg!*** macro, which takes ownership of an expression (as opposed to println!, which takes a reference).

- It prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

-  The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!

</details>

<details> 

<summary> 5.3. Method Syntax </summary>

- ***Methods*** are similar to functions: we declare them with the `fn` keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.

- Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

- The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of `self` in every method’s signature, is for organization. 

- Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do. Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API.

## Associated Functions

- All functions defined within an ``impl block`` are called ***associated functions*** because they’re associated with the type named after the impl. 

- We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this, the *String::from* function that’s defined on the String type.

</details>
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
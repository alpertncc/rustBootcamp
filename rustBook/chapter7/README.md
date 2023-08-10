### Managing Growing Projects with Packages, Crates, and Modules

- As you write large programs, organizing your code will become increasingly important. By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.

- The programs we’ve written so far have been in one module in one file. As a project grows, you should organize code by splitting it into multiple modules and then multiple files.

- Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

- ***IMPORTANT*** : It is necessary to master all the details about this Chapter. My notes will help you quickly remember something you already know. So please go to the [relevant Chapter of RustBook](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) and read the whole thing.

<details> 

<summary> Packages and Crates </summary>

The first parts of the module system we’ll cover are ***packages and crates***.

- A ***crate*** is the smallest amount of code that the Rust compiler considers at a time.

- Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.

- A crate can come in one of two forms: ***a binary crate or a library crate***. 

1. ***Binary crates are programs*** you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.

1. ***Library crates*** don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. 

- The ***crate root*** is a source file that the Rust compiler starts from and makes up the root module of your crate.

- A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

- The Cargo package also contains a library crate that the binary crate depends on.

- A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
- After we run cargo new, we use ls to see what Cargo creates. In the project directory, there’s a Cargo.toml file, giving us a package. There’s also a src directory that contains main.rs. Open Cargo.toml in your text editor, and note there’s no mention of src/main.rs. Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

</details>



<details> 

<summary> Defining Modules to Control Scope and Privacy </summary>

- In this section, we’ll talk about modules and other parts of the module system, namely paths that allow you to name items; the ***use*** keyword that brings a path into scope; and the ***pub*** keyword to make items public. We’ll also discuss the ***as*** keyword, external packages, and the glob operator.

# Modules Cheat Sheet

- - The ***use*** keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with ```use crate::garden::vegetables::Asparagus;``` and from then on you only need to write Asparagus to make use of that type in the scope.

- Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with ``pub mod`` instead of ``mod``. To make items within a public module public as well, use pub before their declarations.


- Here we create a binary crate named ***backyard***;

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

- The crate root file in this case is src/main.rs, and it contains:

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;
#[derive(Debug)]
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

- The ``pub mod garden;`` line tells the compiler to include the code it finds in ***src/garden.rs***, which is: Filename: src/garden.rs

# [Grouping Related Code in Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#grouping-related-code-in-modules)

</details>

<details> 

<summary> Paths for Referring to an Item in the Module Tree </summary>

</details>
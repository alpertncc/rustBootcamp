### Enums and Pattern Matching

<details> 

<summary> Defining an Enum </summary>

- ***IMPORTANT*** => Please check the `chapter/definingEnum/src/main.rs` file to be able to examine the details in code. All the details about this chapter is in the file I said above.

</details>

<details> 

<summary> The match Control Flow Construct </summary>

- ***IMPORTANT*** => Please check the `chapter/theMatch/src/main.rs` file to be able to examine the details in code. 

- Rust has an extremely powerful control flow construct called ***match*** that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

- The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

- Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

</details>

<details> 

<summary> Concise Control Flow with if let</summary>

- ***IMPORTANT*** => Please check the `chapter/ifLet/src/main.rs` file to be able to examine the details in code. 

- The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. 

- The syntax ``if let`` takes a pattern and an expression separated by an equal sign. It works the same way as a ``match``, where the expression is given to the match and the pattern is its first arm. 

- Using ``if let`` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that ``match`` enforces. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

</details>

# Summary

- Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures type safety: the compiler will make certain your functions only get values of the type each function expects.

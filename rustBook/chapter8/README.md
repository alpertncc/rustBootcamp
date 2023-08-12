### Common Collections

- Rust’s standard library includes a number of very useful data structures called ``collections``. Most other data types represent one specific value, but collections can contain multiple values. 

- Unlike the built-in array and tuple types, the data these collections point to is ***stored on the heap***, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

* ``A vector`` allows you to store a variable number of values next to each other.

* ``A string`` is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.

* ``A hash map`` allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

- ***IMPORTANT***: To examine the code examples and learn the topics well, I suggest you to look at the code blocks.

<details> 

<summary> Storing Lists of Values with Vectors </summary>

- The code file is at the `chapter8/vectors/src/main.rs` direction.

- The first collection type we’ll look at is ``Vec<T>``, also known as a ***vector***. Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. 

- They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

</details>

<details> 

<summary> Storing UTF-8 Encoded Text with Strings </summary>

- The code file is at the `chapter8/strings/src/main.rs` direction.

- We’ll first define what we mean by the term ***string***. Rust has only one string type in the core language, which is the string slice ``str`` that is usually seen in its borrowed form ``&str``. In Chapter 4, we talked about string slices, which are references to some *UTF-8 encoded string data stored elsewhere*. ***String literals***, for example, are stored in the program’s binary and are therefore string slices.

- The ``String`` type, which is provided by Rust’s standard library rather than coded into the core language, is a *growable, mutable, owned, UTF-8 encoded* string type. When Rustaceans refer to ***strings*** in Rust, they might be referring to either the ``String`` or the string slice ``&str`` types, not just one of those types. 

- Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

</details>

<details> 

<summary> Storing Keys with Associated Values in Hash Maps </summary>

- The code file is at the `chapter8/hashMaps/src/main.rs` direction.

- The last of our common collections is the hash map. The type ``HashMap<K, V>`` stores a mapping of keys of type ``K`` to values of type ``V`` using a ***hashing function***, which determines how it places these keys and values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

- Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type. For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score. Given a team name, you can retrieve its score.

</details>

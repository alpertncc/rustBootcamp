fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        // `println!("{}", s);` : This will print `hello, world!`
    }

    {
        let x = 5;
        let y = x; //because the integers are simple values 
        // with a known, fixed size, `let y = x;`creates a copy of 'x' on the stack.
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        // This piece of code will create a 'shallow copy' of 's1'.
        // 's2' will have the same pointer to 's1' on the heap. When 's1' goes to out of scope,
        // 's2' will also be dropped. This can lead to memory corruption.
        // After the line `let s2 = s1;`, Rust consders 's1' as no longer valid to make safe the memory.
        // To see the error, run the scope with this line: `println!("{}, world!", s1);`
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        // With the 'clone' method, 's2' is no longer using the same pointer with 's1'.
        // They have completely different data and the pointer on the heap. 
        // You can use both variables at the same time. 
        // To see the output, run the scope with this line:  `println!("s1 = {}, s2 = {}", s1, s2);`

    }



}

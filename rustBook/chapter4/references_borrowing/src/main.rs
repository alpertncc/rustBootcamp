{
    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1); //  We pass "&s1" into "calculate_length" and, in its definition, we take "&String" rather than "String".

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, it is not dropped.

}

{
    //If we try to change the value of a variable that we borrowed, it DOES NOT WORK!
    fn main() {
        let s = String::from("hello");
    
        change(&s);
    }
    
    fn change(some_string: &String) {
        some_string.push_str(", world");
    }
    // Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
    // We should use `Mutable References` to be able to change the variable we borrowed.
}

{
    fn main() {
        let mut s = String::from("hello");
    
        change(&mut s);
    }
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // Mutable references have one big restriction: 
    // if you have a mutable reference to a value, you can have no other references to that value.
}

{
    fn main() {
        let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s;

        // If you try to run this line: `println!("{}, {}", r1, r2);`, this will cause an error.
    }    
}

{
    fn main() {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM

        // To see the error, run this line: `println!("{}, {}, and {}", r1, r2, r3);`
        //  We also cannot have a mutable reference while we have an immutable one to the same value.
    }
}
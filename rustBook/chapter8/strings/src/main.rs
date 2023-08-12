fn main() {
    
    // Creates a new, empty String
    let mut s = String::new();


    {
        let data = "initials";
        let s = data.to_string();
        // Using the to_string method to create a String from a string literal

        let s = "initial data".to_string();
    }
    
    // Creates a new String
    let s = String::from("initial data");

    {
        
        let mut s = String::from("foo");
        s.push_str("bar"); // Appending a string slice to a String using the push_str method

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}"); // Using a string slice after appending its contents to a String

        let mut s3 = String::from("lo");
        s3.push('l'); // The push method takes a single character as a parameter and adds it to the String.
    
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        //  we can only add a &str to a String; we can’t add two String values together.
        // s2 has an &, meaning that we’re adding a reference of the second string to the first string. 

    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
    
        let s = s1 + "-" + &s2 + "-" + &s3;
    
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
    
        let s = format!("{s1}-{s2}-{s3}");
        // The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
    
    }


}

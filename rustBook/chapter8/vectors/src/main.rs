fn main() {

    let v: Vec<i32> = Vec::new(); // Creates a new, empty Vector.
    // Because we aren’t inserting any values into this vector,
    // Rust doesn’t know what kind of elements we intend to store. 

    let mut v = vec![1, 2, 3];
    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.

    v.push(5);
    v.push(6);
    // `push` method can be used to add elements to a vector.
    // The vector `v` should be `mutable` to be able to push an element.
    v.push(7);
    v.push(8);

    let third : &i32 = &v[2]; // Using & and [] gives us a reference to the element at the index value.
    println!("The third element is {third}");

    let third : Option<&i32> = v.get(2);
    // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    {
        // To access the elements in a vector, we can use for loop. 
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }    

    }

    {
        // We can change the elements in a MUTABLE vector with for loop. 
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        // To change the value that the mutable reference refers to, we have to use the * dereference operator 
        // to get to the value in i before we can use the += operator.
        }

    }

    {
        // Vectors can only store the same type of elements. 
        // By using enum, we can put different types of elements into a vector.  
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    
    }

    {
        {
            let mut v = vec![1, 2, 3, 4];
            v.pop();
            // pop method removes and returns the last element.
        } 
    
    }

}
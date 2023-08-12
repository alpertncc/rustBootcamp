
use std::collections::HashMap;

fn main() {
    
    let mut scores = HashMap::new();
    // Creating a new hash map and inserting some keys and values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // Accessing the score for the Blue team stored in the hash map,
    // The get method returns an Option<&V>,
    // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, 
    // then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    for (key, value) in &scores {
        println!("{key}: {value}");
        // The output is,  Yellow: 50  Blue: 10

    }

    {

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        // Replacing a value stored with a particular key
    
        println!("{:?}", scores);
        
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        // Hash maps have a special API for this called entry that takes the key you want to check as a parameter. 
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("{:?}", scores);
        // {"Yellow": 50, "Blue": 10}. 
    
    }

}


{
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn main() {

    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

}

{
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn main() {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }
}


{

// Matching with Option<T>


    fn main() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five); // The i binds to the value contained in Some, so i takes the value 5. 
                                  // The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
        let none = plus_one(None); // There’s no value to add to, so the program stops and returns the None value on the right side of =>. 
                                   // Because the first arm matched, no other arms are compared.
    }

}



{
    // Catch-all Patterns and the _ Placeholder

    // the result of the dice roll hardcoded rather than a random value, 
    // and all other logic represented by functions without bodies.
    fn main() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
            // If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }

}

{
    fn main() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(), // _ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t 
                           // going to use the value, so Rust won’t warn us about an unused variable.
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }

}

{
    fn main() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (), // Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern 
                     // in an earlier arm, and we don’t want to run any code in this case.
        }
    
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
    }

}

{
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max), 
        // If the value is Some, we print out the value in the Some variant by binding the value to the variable max in the pattern.
        _ => (),
    }
}
}


// The syntax if let takes a pattern and an expression separated by an equal sign.
// It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

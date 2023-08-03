// A program to calculate the area of a rectangle

{
    // Calculating the area with the basic variables
    fn main() {
        let width1 = 30;
        let height1 = 50;
    
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }
    
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

{
    // Refactoring with Tuples
    fn main() {
        let rect1 = (30, 50);
    
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
    }
    
    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

{
    // Refactoring with Structs

    struct Rectangle {
        widht: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {
            widht: 30,
            height: 50,
        };

        println!("The area of the rectangle is {} square pixels.",
        area(&rect1)
        );
    }

    fn area(rectangle: &Rectangle) -> u32 { 
        // we want to borrow the struct rather than take ownership of it. 
        // This way, main retains its ownership and can continue using rect1, 
        // which is the reason we use the & in the function signature and where we call the function.
        rectangle.width * rectangle.height
    }

}







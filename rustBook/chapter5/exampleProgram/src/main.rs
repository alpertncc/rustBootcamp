
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
        // This way, main function retains its ownership and can continue using rect1, 
        // which is the reason we use the & in the function signature and where we call the function.
        rectangle.width * rectangle.height
    }

}




// Adding Useful Functionality with `Derived Traits`
{
    #[derive(Debug)] // You MUST add this line to be able to take output of the structs!
    struct rectangle {
        widht: u32,
        height: u32,
    }

    
    fn main() {
        let rect5 = rectangle {
            widht: 30,
            height: 50,
        };

        // When you run this line: `println!("rectangle is {}", rect5);`
        // you will take the error, "`rectangle` cannot be formatted with the default formatter"
        // This is because, println! macro can display the defined traits before. 
        // You should specify the display format! 

        println!("rectange is {:?}",rect5); // The output will be `rectange is rectangle { widht: 30, height: 50 }`

        //  When we have larger structs, it’s useful to have output that’s a bit easier to read;
        //  in those cases, we can use ` println!("rectange is {:#?}",rect5);`. The output will be; 
        /* `rect1 is Rectangle {
                width: 30,
                height: 50,
            }`
        */


    }
}

{
        // dbg! macro 

        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        
        fn main() {
            let scale = 2;
            let rect1 = Rectangle {
                width: dbg!(30 * scale), // You will take an output from this line!
                height: 50,
            };
        
            dbg!(&rect1); // You will take another output from this line!
            // We used "&" because we do NOT want `dbg!` to take ownership of "rect1", so we use a reference!
        }
}   







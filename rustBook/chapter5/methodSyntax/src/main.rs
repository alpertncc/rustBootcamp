// Defining Methods

{
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle { // Everything within this impl block will be associated with the Rectangle type.
        fn area(&self) -> u32 {  // We don’t want to take ownership of the Self variable.
            // If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use "&mut self" as the first parameter.
            self.width * self.height
        }
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("The are of the rectangle: {}", 
        rect1.area())
    };

}

{
    // Methods with More Parameters

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    fn main(){
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

}



// Associated Functions
{
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    
    fn main() {
        let sq = Rectangle::square(3); // To call this associated function, we use the :: syntax with the struct name
        println!("{:?}",sq)
    }    
    
}
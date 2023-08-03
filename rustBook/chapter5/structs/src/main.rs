struct User {  // To define a `Struct`, we are using this syntax. 
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

{   fn main() {
     let mut user1 = User {  // You should add `mut` to make changable a variable!
         active: true,
         username: String::from("BlockofChain"),
         email: String::from("blockofchain@example.org"),
         sign_in_count: 1,
     };

     // You can change a veriable bu calling is: user1.email = String::from("example@blockfochain.org");

     let user2 = User {   // With the `Struct Update Syntax`, you can define another User by using the old user's data.
         active: true,  // By using `Struct Update Syntax`, you can use less code.
         username: user1.username,
         email: String::from("odtublockchain@exapmle.com"),
         sign_in_count: user1.sign_in_count,
     };

     let user3 = User {   // With the `..` syntax, you can specify that the remaining fields not explicitly set 
                          // should have the same value as the fields in the given instance.
         email: String::from("example@odtublockchain.com"),
         ..user2 // !!!REMEMBER: Due to ownership rules, you can NOT take the values from `user1` because we used the values of `user1` while defining the `user2`!!!
         // You must use the `..` syntax at the end of the struct!
     };

}   

    fn  build_user(email: String, username: String) -> User {
     User {
         active: true,
         username, // You can use `field init shorthand` syntax to define
         email, // the parameters that we put inside the function `build_user`
         sign_in_count: 1,
     }
}   

} 

// `Tuple Structs`

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

{
    fn main() {
        let black = Color(0,0,0);
        let origin = Point(0,0,0);
        // `black` and `origin` values are different types because they're instances of different tuple structs.
    }

}

// Unit-Like Structs Without Any Fields

struct AlwaysEqual;

{
    fn main() {
        let subject = AlwaysEqual; // No need for curly brackets or parentheses.
        //  Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every 
        //  instance of any other type, perhaps to have a known result for testing purposes.
    }
}
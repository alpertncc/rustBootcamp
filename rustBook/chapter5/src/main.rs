struct User {  // To define a `Struct`, we are using this syntax. 
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {  // You should add `mut` to make changable a variable!
        active: true,
        username: String::from("BlockofChain"),
        email: String::from("blockofchain@example.org"),
        sign_in_count: 1,
    };

    // You can change a veriable bu calling is: user1.email = String::from("example@blockfochain.org");

    let mut user2 = User {   // With the `Struct Update Syntax`, you can define another User by using the old user's data.
        active: true,
        username: user1.username,
        email: String::from("odtublockchain@exapmle.com"),
        sign_in_count: user1.sign_in_count,
    };

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // You can use `field init shorthand` syntax to define
        email, // the parameters that we put inside the function `build_user`
        sign_in_count: 1,
    }
}




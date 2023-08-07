
{    
    enum IpAddrKind { // Defining an enum
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    fn main() {
        let four = IpAddrKind::V4; // Note that the variants of the enum are namespaced under its identifier, 
        let six = IpAddrKind::V6;  // and we use a double colon to separate the two.

        route(IpAddrKind::V4);
        route(IpAddrKind::V6);

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    fn route(ip_kind: IpAddrKind) {

    }
}

{
    // Rather than an enum inside a struct, we can put data directly into each enum variant.
    enum IpAddr {
        V4(String),
        V6(String),
    }

    fn main() {

        let home = IpAddr::V4(String::from("127.0.0.1"));
        // We attach data to each variant of the enum directly, so there is no need for an extra struct.
        let loopback = IpAddr::V6(String::from("::1"));
    }
}

{
    // There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    fn main() {

    
        let home = IpAddr::V4(127, 0, 0, 1);
        // We attach data to each variant of the enum directly, so there is no need for an extra struct.
        let loopback = IpAddr::V6(String::from("::1"));
    }
}

{
    enum Message {
        Quit, // has no data associated with it at all.
        Move { x: i32, y: i32 }, // has named fields, like a struct does.
        Write(String), // includes a single String.
        ChangeColor(i32, i32, i32), // includes three i32 values.
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    fn main() {
        let m = Message::Write(String::from("Hello")); 
        m.call();
    }
    
}
 


{
// ******** The Option Enum and Its Advantages Over Null Values ********


    fn main() {
        enum Option<T> { // <T> means that the Some variant of the Option enum can hold one piece of data of any type.
            None,
            Some(T), 
        }
        
        let some_number = Some(5); // T is i32 type
        let some_char = Some('e'); // T is char type
    
    }
}

fn main() {
    // An Enum instance value can only be one of it's variants like a radio list 
    // Instead of no associated typed value to be a variant on a struct field
    // Enums variants can bind a typed value

    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    /*
        Using either Structs or Enums is a design decision considering they can
        be used in conjunction as exemplified from the implementation of 
        IpAddr in the standard library:
    */

    struct Ipv4Addr {

    }

    struct Ipv6Addr {

    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr)
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    // You can incoke impl on Emums as well to create instance methods
    impl Message {
        fn call(&self) {

        }
    }
}

/*
    Options are widely used enums to exemplify nullable values 
    There are are cases when you'd want the type Some, None, <T> 
    When working with them 
*/
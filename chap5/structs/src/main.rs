
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}



fn main() {
    /*
        What is a Unit-like struct?
        What is a trait? What is a lifetime? What do 
        these have to do with struct references? see Chapter 10

        A Struct is a collection of named data attributes much like with objects
        Like tuples they can store different types but of multiple related values
        to not rely on order to specify/access an instance.

        Each item must all be mutable within an instance or not. 

    */

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("Username123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("updatedemail@example.com");

    /*
        Tuple Structs have the added meaning the struct name provides but don't 
        have names associated with their fields; they just have the types of the fields
        from a named collection 
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32 , i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

// constructor pattern for default values 
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// extend pattern "Creating Intances from other instances" see whether or not this 
// invalidates the initial value instance
fn extends_user(email: String, user: User) -> User {
    User {
        email,
        ..user
    }
}
fn main() {
/*
    Ownership is a bit like "closures" and factor of mutability to know whether 
    or not a variable's size is known at compile since it could change while running the program

    The basic Scalar and Compound (arrays etc..) are all stored on the stack and 
    popped off the stack when their scope is over, but we want to look
    at data that is stored on the heap and explore how Rust knows when
    to clean up that data.

    Here we demonstrate this with how a string literal cannot be mutated but instead how a string
    the String module can:

*/
    let s = "hello";

    let mut s_from_heap = String::from("hallo");

    s_from_heap.push_str(", worlds");

    println!("{}", s_from_heap) // format since using variable name

}

/*
    In the case of a string literal, we know the contents at compile time, 
    so the text is hardcoded directly into the final executable.

    There is a natural point at which we can return the memory our String 
    needs to the allocator (automatically): when "s_from_heap" goes out of scope. 
    When a variable goes out of scope, Rust calls a special function for us. 
    This function is called "drop", and it’s where the author of String can put 
    the code to return the memory. Rust calls drop automatically at the closing 
    curly bracket.

    Note: In C++, this pattern of deallocating resources at the end of an item’s 
    lifetime is sometimes called Resource Acquisition Is Initialization (RAII).
    The drop function in Rust will be familiar to you if you’ve used RAII patterns.


    Types that implement the trait Copy allow for

*/

/*

    &str vs String type 

*/
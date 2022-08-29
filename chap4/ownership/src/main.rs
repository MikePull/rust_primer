fn main() {
/*
    Ownership is a bit like "closures" while also considering mutability to know whether 
    or not a variable's size is known at compile time since it could change while running the program

    The stack stores values in the order it gets them and removes the values in the opposite order. 
    This is referred to as last in, first out

    Values of known size are stored on the stack while values of unknown size (mutated by the user?)
    are stored on the heap. "Memory allocation" is done by pointers to a memory address for the heap.
    because the pointer is a known, fixed size, you can store the pointer on the stack, 
    but when you want the actual data, you must follow the pointer.

    Think of being seated at a restaurant. When you enter, you state the number of people in your group, 
    and the staff finds an empty table that fits everyone and leads you there.
    If someone in your group comes late, they can ask where you’ve 
    been seated to find you. 

    This exemplifies the difference in speed to store and look up data on the stack and heap
    Taking this further the proccessor would do its job better by taking all the orders from
    one location before moving on.

    The basic Scalar and Compound types (arrays etc..) are all stored on the stack and 
    popped off the stack when their scope is over, but we want to look
    at data that is stored on the heap and explore how Rust knows when
    to clean up that data.

    Here we demonstrate this with how a string literal cannot be mutated but instead how a string
    the String module can:

*/
    let s = "hello";

    let mut s_from_heap = String::from("Hello");

    s_from_heap.push_str(" worlds");

    println!("{}", s_from_heap) // format since using variable name

}

/*

    // What is "drop"
    // What is double free?
    // What is a "move" 

    In the case of a string literal, we know the contents at compile time, 
    so the text is hardcoded directly into the final executable.

    There is a natural point at which we can return the memory our String 
    needs to the allocator (automatically): when "s_from_heap" goes out of scope.

    When a variable goes out of scope, Rust calls a special function for us. 
    This function is called "drop", and it’s where the author of String can put 
    the code to return the memory. Rust calls drop automatically at the closing 
    curly bracket.

    Reassigning any variables from the stack is the equivalent of "shallow copying a deep reference
    to the the memory location"

    Note: In C++, this pattern of deallocating resources at the end of an item’s 
    lifetime is sometimes called Resource Acquisition Is Initialization (RAII).
    The drop function in Rust will be familiar to you if you’ve used RAII patterns.

    Garbage Collection historically through runtimes as been from matching each "allocate"
    to "free" calls upon catching memory through events. 

    These configurations by which to exemplify Rust's memory safety through the allocator. 

    the invalidation step as seen in compilers is refered to as a "move" of addresses as opposed to 
    shallow v. deep copying ("cloning" see chapter 5)
*/

/*

    &str vs String type 


*/


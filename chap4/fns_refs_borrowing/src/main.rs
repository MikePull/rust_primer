fn main() {
    /*
        "What is a non-lexical lifetime?"

        When variables go in and out of scopes their ownership changes based on closure as 
        well as if a reference mutuable or not as in to give ownsership back upon a return value

        "Luckily for us, Rust has a feature for using a value without transferring ownership,
        called references. A reference is guaranteed to point to a valid value of a particular type
        for the life of that reference as a memory address that can be sourced.
    */
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is '{}' ", s1, len);
}

fn calculate_length(s: &String) -> usize { // a reference that is not "dropped"
    s.len()
} 
// s --> s1 --> memory address
fn change(a_string: &mut String) {

    // mutable references get passed in end to end from 
    // instantiation to even give syntax guidance making it clear
    // that a function mutates the value it borrows. 
    // You can only have one mutable reference to a value at a 
    // time within a given scope unless there will be an error. 
    // Improper use can result in a data race: 

    /*
    When: 
        "
        Two or more pointers access the same data at the same time.
        At least one of the pointers is being used to write to the data.
        There's no mechanism being used to synchronize access to the data. 

        Rust refuses to compile code with data races
        "
        There are multiple ways of having multiple references see non lexical lifetimes 
        and examples

        What is a dangling pointer? See returning a borrowed value instantiated within 
        that source function.
    */

    a_string.push_str(" world");
    println!("changed a reference passed in to {}", a_string);
}
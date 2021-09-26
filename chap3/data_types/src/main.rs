fn main() {
    /*
        Rust is a statically typed language, 
        which means that it MUST know the types 
        of all variables at compile time. 
        The compiler can usually infer what type we want 
        to use based on the value and how we use it.
    */
    /* 
        There are four main Scalar Types:
            int, float, bool, char
    */
    /* numbers => 
        Signed and unsigned refer to whether it’s possible 
        for the number to be negative or positive—in other 
        words, whether the number needs to have a sign with it 
        (signed) or whether it will only ever be positive and 
        can therefore be represented without a sign (unsigned).

        Signed numbers are stored using two’s complement representation.
        The primary situation in which you’d use isize or usize is when 
        indexing some sort of collection.
    */

    /* floats => 
        Rust also has two primitive types 
        for floating-point numbers, which are 
        numbers with decimal points. Rust’s 
        floating-point types are f32 and f64, 
        which are 32 bits and 64 bits in size, 
        respectively. The default type is f64 because 
        on modern CPUs it’s roughly the same speed as 
        f32 but is capable of more precision.
    */
    // The char type is usually implicit
/////////////////////////////////
    /* 
        There are two main Compound Types:
            tuple, array
    */
    /*
        A tuple is a general way of grouping together 
        a number of values with a variety of types into 
        one compound type. Tuples have a fixed length: once declared, 
        they cannot grow or shrink in size.
    */
    let tup: (i32, f64, u8) = (400, 8.4, 5);
    /*
        The variable tup binds to the entire tuple, 
        because a tuple is considered a single compound element. 
        To get the individual values out of a tuple, we can use 
        pattern matching to destructure a tuple value, like this:
    */  
    let (x, y, z) = tup;

    println!("The value of z is {}", z);

    /*
        In addition to destructuring through pattern matching, 
        we can access a tuple element directly by using a period (.) 
        followed by the index of the value we want to access:
    */

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    /* 
        Arrays are useful when you want your data allocated on 
        the stack rather than the heap or when you want to ensure 
        you always have a fixed number of elements.
        
        An example of when you might want to use an array rather than a vector 
        is in a program that needs to know the names of the months of the year:
    */

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    /*
        You would write an array’s type by using square brackets, 
        and within the brackets include the type of each element, 
        a semicolon, and then the number of elements in the array, like so:
    */

    let my_typed_array : [i32; 6] = [2, 4, 6, 7, 34, 5];
}
fn main() {
    add_two_nums(5, 8);

    eval_expression(89);

    println!("This is my function that returns: {}", five());
}

// This is a basic function taking in params
fn add_two_nums(x: i32, y: i32) {
    println!("The value of the sum is: {}", x + y);
}

// This is a function that contains an expression using {}
fn eval_expression(val: i32) {
    let y = {
        let x = 3;
        x + val
    };
    println!("{}", y);
}

// This is a function that implicitly returns the last value 
// (DON'T PUT A SEMI-COLON IF USING IMPLICIT RETURN 
fn five() -> i32 {
    5
}

/*
    In function signatures, you must declare the type of each parameter. 
    This is a deliberate decision in Rust’s design: requiring type annotations 
    in function definitions means the compiler almost never needs you to use 
    them elsewhere in the code to figure out what you mean.    
*/
/* 
    We don’t name return values, but we do declare their type after an arrow (->). 
    In Rust, the return value of the function is synonymous with the value of 
    the FINAL EXPRESSION in the block of the body of a function. 
    You can return early from a function by using the return keyword 
    and specifying a value but most functions return the last expression implicitly.
*/
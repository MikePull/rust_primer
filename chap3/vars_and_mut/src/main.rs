fn main() {
    /*
        Mutability can be very useful. 
        Variables are immutable only by default;
        You can make them mutable by adding mut in front of the variable name. 

        In addition to allowing this value to change, 
        mut conveys intent to future readers of the code 
        by indicating that other parts of the code will be changing 
        this variable’s value.

        From an event-driven standpoint and taking from "static" "public" and "private" keywords 
        Mutation can always be looked at through the dichotomy of pipelining a user's inputs and outputs
        to a state tree before being submitted within a payload. 

    */
    let mut x = 5;
    println!("The value of x is, {}", x);
    // MUTATION
    x = 6;
    println!("The value of x is, {}", x);

    // An example of a "constant" constants MUST have a type defined to them
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // Shadowing a value:
    let shadowed_val = 5;
    let shadowed_val = shadowed_val + 1;
    let shadowed_val = shadowed_val * 2;

    /*

        Shadowing a value is when it can be easily overwritten using the let keyword or not. 

    */

    println!("The shadowed value is: {}", shadowed_val);
    
    /* 

        By using let, we can perform a few 
        transformations on a value but have the 
        variable be immutable AFTER those transformations 
        have been completed.
   
    */

    /* 
    
        Constants

    */
}

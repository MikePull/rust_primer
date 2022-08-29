use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // SEE EVENT DRIVEN PROGRAMMING

    // Use of the println Macro 
    println!("Guess the number!");`
     /*
        retrieves the lazily-initialized thread-local random number generator, 
        "seeded" by the system. Intended to be used in method chaining style, 
        e.g. thread_rng().gen::<i32>
        
    */
    let secret_number = rand::thread_rng().gen_range(1..101); // takes in specific type
   
    // comment out this line for debugging and to not give away the number.
    println!("This is the secret: {}", secret_number);

    // loops forever breaking at keywords
    loop {
        println!("Please input your guess.");

        // mut makes variables mutable see Chapter 3 "guess" is bound to a new empty instance of String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        // This casts the string to an int after parsing the input as a string 
       let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
       };

       /* 
        BEFORE: let guess: u32 = guess.trim().parse().expect("Please type in a number!");      
        AFTER: (IF parse() is successful or not)

            Switching from an expect call to a match expression is 
            how you generally move from crashing on an error to HANDLING the error. 
            Remember that parse returns a Result type and Result is an enum 
            that has the variants Ok or Err. We’re using a match expression here, 
            as we did with the Ordering result of the cmp method.
        */

        println!("You guessed: {}", guess);

        // Why does cmp only take a reference??

        // This will make whatever is returned, return forever 
        // https://stackoverflow.com/questions/48094170/when-to-use-stdcmpordering-instead-of-an-if-statement-in-rust
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        };
   }
}
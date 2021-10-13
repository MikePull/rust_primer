use std::io;
use std::any::type_name;

fn main() {

    // If and If-else statements are standard in Rust; the conditional needs no parentheses
    basic_if_else();

    // There are Infinite Loops (as seen in Chapter 2 using a prompt), For Loops, and While Loops
    learn_loops();
    
    fizzbuzz(
        loop_readline("Input a number for fizzbuzz")
    );

// INFINITE LOOP:

    // loop {
    //     println!("again! STOP by pressing CTRL + C")
    // }

// You can also return a value from a loop using an assignment and "break" keyword. 
        
}

fn basic_if_else () {
    println!("From basic_if_else fn:");

    let num = 3;
    if num < 5 {
        println!("condition is true number less than 5");
    } else {
        println!("condition is false number greater than or equal to 5");
    }
    println!("----------------")

     // Using if w/ let to make a conditional assignment: 
     let condition = true;
     let my_number = if condition { 5 } else { 6 }; // INFERRED TYPES CANNOT BE SWTICHED
     println!("-------- \nFrom the conditional assignment: {} ", my_number);
}

fn learn_loops() {

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}

fn fizzbuzz (num: u32) {
    if num % 3 == 0 && num % 5 == 0 {
        println!("Multiple of 3 --- ({}) & 5 --- ({}) FizzBuzz!", num / 3, num / 5)
    } else if num % 3 == 0 {
        println!("Multiple of 3 --- ({}) Buzz!", num / 3);
    } else if num % 5 == 0 {
        println!("Multiple of 5 --- ({}) Fizz!", num / 5);
    } else {
        println!("Neither multiple of 5 nor 3");
    }
}


// TODO -> Change take in parameter to expect a generic type
fn loop_readline(prompt: &str) -> u32 {
    let mut _input = String::new();
    loop {
        println!("{}", prompt);

        io::stdin()
                .read_line(&mut _input)
                .expect("Failed to read line");
        
        let _input: u32 = match _input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

// fn type_of(_: T) -> &'static str {
//     type_name::()
// }
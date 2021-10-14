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

    ////////////////// EXERCISES ///////////////////
    convert_temp(1.0, "F");
    gen_nth_fibonacci_loop(5);

}

fn basic_if_else () {
    println!("From basic_if_else fn:");

    let num = 3;
    if num < 5 {
        println!("condition is true number less than 5");
    } else {
        println!("condition is false number greater than or equal to 5");
    }
    println!("----------------");

     // Using if w/ let to make a conditional assignment: 
     let condition = true;
     let my_number = if condition { 5 } else { 6 }; // INFERRED TYPES CANNOT BE SWTICHED
     println!("-------- \nFrom the conditional assignment: {} ", my_number);
}

fn learn_loops() {
// You can also return a value from a loop using an assignment and "break" keyword. 
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

   println!("{}", result);

// While loops are conventional like other languages:

    let mut inc = 3;
    while inc != 0 {
        println!("{}!", inc);

        inc -= 1
    }
    println!("LIFTOFF!!! (while loop)");

// For loop by traversing an array:
    let arr = [10, 20, 30, 40, 50];

    for el in arr.iter() {
        println!("The value is: {}", el);
    }

// Using an arbitrary "Range":
    for num in (1..4).rev() {
        println!("{}", num);
    }
    println!("LIFTOFF!!!!")
}

fn fizzbuzz (num: u32) {
    if num % 3 == 0 && num % 5 == 0 {
        println!("Multiple of 3 --- ({}) & 5 --- ({}) FizzBuzz!", num / 3, num / 5);
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

/////////////// EXCERCISES ////////////////

// Convert temperatures between Fahrenheit and Celsius.
// Better to use a struct to switch between formats
fn convert_temp(mut temp: f32, mut format: &str) -> f32 {

    let original_temp = temp;

    if format == "C" {
        temp = temp * 1.8 + 32.0;
    } else if format == "F" {
        temp = (temp - 32.0) / 1.8;
    }

    println!("{} degrees was converted to {} {}deg", original_temp, temp, format);
    temp
}

// Generate the nth Fibonacci number. (0, 1, 1, 2, 3, 5 etc.... n)

fn gen_nth_fibonacci_loop (n: usize) -> usize { 
    println!("----------\nFrom computing the {}th fibonacci number using a loop instead of recursion", n);
    let mut x = 0;
    let mut y = 1;

    for _ in 1..n {
        println!("{}", y);
        let prev = x;
        x = y;
        y += prev;
    }

    println!("The {}th fibonacci number is {} value returned", n, y);

    y
}


// fn gen_nth_fibonacci_recursive(n: i32, x: Option<i32>) -> i32 {
//     let mut _x = x.unwrap_or(1);

//     println!("{}", _x);

//     if n == 0 {
//         return _x;
//     } else {
//         return _x + gen_nth_fibonacci_recursive(n - 1, _x); 
//     }
// }
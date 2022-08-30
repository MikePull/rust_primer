fn main() {
    /*
        Match allows you to compare a value against a series 
        of patterns and then execute code based on which pattern
        matches.

        Each code block to execute for each pattern is formatted
        <pattern> => <codeblock> where codeblock can be a line or in
        curly braces { }

        Enums can have defualt values as well as no ops with shorthand 
        _ => 

        Matching with Option<T> is common
        
        An enum variant can also take an input of a given type as seen with 
        Quarter(UsState)
    */

    enum Coin {
        Penny, 
        Nickel,
        Dime,
        Quarter(UsState),
    }

    enum UsState {
        Alabama,
        Alaska, 
        Arkansas,
        Washington,
        Utah,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => {
                println!("Thats a dime");
                10
            },
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state)
            },

        }
    }
}

fn main() {
    /*
        What is 'deref coercion' see Chapter 15
        String literals are slices which are immutable references 
        The type '&str' is an immutable reference.

        A string slice -> &s[<start index>..<end index>] 
        ommitting an index for the rest from the start/end

        A slice of an array of integers is a 'Vector' covered in Chapter 8

        Consider two versions on a function that returns the 
        first word or whole word if there are no spaces.
    */

    let mut s = String::from("hello world");
    let word = first_word_no_slice(&s);
    s.clear(); //The String and the reference are no longer bound


}

/*

    Iterates over that array as a sequence of bytes but refers to the entire array
    rather than the portion

*/

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // Iterators in Chapter 13 Enum in Chap 6
        if item == b' ' {
            return i
        }
    }
    s.len()
}

fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}
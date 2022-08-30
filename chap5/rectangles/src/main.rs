// Can be refactored with Tuples and Structs see the Debug Trait to inspect state
// Methods can be declared using impl block as well as 'self' keyword can be used
// for member values / methods from taking &self as parameter mutably or not 

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
///////////////////
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 contain rect2? {}", rect1.can_contain(&rect2));
    println!("Can rect2 contain rect3? {}", rect2.can_contain(&rect3));
    
    let sq = Rectangle::square(4)
}

struct Rectangle {
    height: u32,
    width: u32
}

// New instances of Rectangle "implement" these methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Modules will be discussed in Chapter 7 but 
// Structs can have associated functions that construct
// or return Self as a sub type of the initial struct accessed from using "::"

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
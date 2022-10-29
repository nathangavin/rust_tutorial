fn main() {

    /*
    ============ REFERENCES ==================
    */
    
    // string literal value is known at compile time, stored on stack
    let s = "hello";
    println!("{}", s);

    // using String complex type allows for mutable strings. stored on heap. value does not need to be known at compile time
    let mut s2 = String::from("hello");

    s2.push_str(", world!");

    println!("{}", s2);

    let s3 = String::from("hello");
    let s4 = s3;
    println!("{}", s4);
    // s3 is now unavailable, as the linked data has been moved to s4
    // println!("{}", s3); would throw an error

    // to do a deep clone of a variable and allow the first to continue being used, use .clone()
    // .clone() is expensive
    let s5 = String::from("hello");
    let s6 = s5.clone();
    println!("{}", s6);

    // primitives are stored on the stack entirely, and so can be copied fine
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    /*
        All data types which store data on the heap, like String, implement the Drop trait
        All primitives implement the Copy trait
        Things which implement the Copy trait can be stored on the stack
        Something cannot implement Copy if it or a child already implements Drop

        for example, a tuple (i32, i32) will implement Copy
        but (i32, String) will not
    */

    /*
        passing a variable to a function as a parameter is the same as assigning it to a different 
        variable. 
        behaviour remains the same for both primitives and complex types
    */

    /*
        we can use 'borrowing'to send a variable via reference to a function, while maintaining 
        ownership of it.

        using & to send reference (pointer) to object
    */
    let s7 = String::from("hello");
    let len = calculate_length(&s7);
    println!("string {} with length {}", s7, len);


    /*
        mutable borrows are done as you would expect.
        Note only 1 mutable reference can exist at once
    */

    let mut s8 = String::from("hello");
    
    let r1 = &mut s8;
    // let r2 = &mut s8; this would throw an error

    // scope can be used to have multiple mut references
    let mut s9 = String::from("hello");

    {
        let r2 = &mut s9;
        println!("{}", r2);
    }
    let r3 = &mut s9;
    println!("{}", r3);


    /*
        a variable cannot have both immut and mut references at once
    */

    let mut s10 = String::from("hello");
    let r4 = &s10;
    let r5 = &s10;
    // let r6 = &mut s10; this is invalid as we cannot have the value of a immut reference change

    // however, if we consume the references and then stop using them
    println!("{}, {}", r4, r5);

    // we can now create a mut reference, as the compiler works out we arent using the immut references anymore
    let r6 = &mut s10;
    println!("{}", r6);

    /*
        ============== SLICES ===============
    */

    let mut s11 = String::from("hello world");
    let s11_word = first_word(&s11);
    s11.clear();
    println!("{}", s11_word);


    // slices allow us to refer to sections of arrays and strings

    let s12 = String::from("hello world");
    let hello = &s12[0..5];
    let world = &s12[6..11];

    // the following are both the same
    // let slice = &s12[0..2];
    // let slice = &s12[..2];

    // similarly, the following are both the same

    // let len = s12.len();
    // let slice = &s12[3..len];
    // let slice = &s12[3..];

    // this selects the whole range
    // let slice = &s12[..];


    

}

fn calculate_length(s: &String) -> usize {
    /*
        by default, we cannot modify a variable we are only borrowing
    */
    s.len()
}

/*
    Example of a function which returns the end position of the first word
    in a string. searches for the first instance of the space character, otherwise
    returns the length of the string if no space.

    This works, but causes a disconnect when doing the following:

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    word still contains the value of 5, even though the string s is now empty
    this creates a disconnect between variables, which have to be manually tracked together.

    Slices solve this
*/
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
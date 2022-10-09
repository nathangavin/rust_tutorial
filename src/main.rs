fn main() {

    /*
    ============ REFERENCES ==================
    */
    
    // string literal value is known at compile time, stored on stack
    let s = "hello";

    // using String complex type allows for mutable strings. stored on heap. value does not need to be known at compile time
    let mut s2 = String::from("hello");

    s2.push_str(", world!");

    println!("{}", s2);

    let s3 = String::from("hello");
    let s4 = s3;
    // s3 is now unavailable, as the linked data has been moved to s4
    // println!("{}", s3); would throw an error

    // to do a deep clone of a variable and allow the first to continue being used, use .clone()
    // .clone() is expensive
    let s5 = String::from("hello");
    let s6 = s5.clone();

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
    }
    let r3 = &mut s9;



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

    /*
        ============== SLICES ===============
    */

    

}

fn calculate_length(s: &String) -> usize {
    /*
        by default, we cannot modify a variable we are only borrowing
    */
    s.len()
}
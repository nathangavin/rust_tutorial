fn main() {
    
    /*
        variables can only be changed if they are mut
    */
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    /*
        const variables are forever immutable and type must be
        explicitly stated, and must be given a complie time value 
        cannot be computed at runtime
        
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Const THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING

    /*
        The following is valid, and is called shadowing. Its different to
        mut because you are reassigning y. y is changed but remains immutable
    */
    let y = 5;
    let y = y + 5;

    {
        /*
            shadowing can be used in scope to redeclare the variable, only to that
            scope
        */
        let y = y * 2;
        println!("y in scope = {y}");
    }
    println!("y out of inner scope = {y}");

    // shadowing 2 - changing type

    /*
        in this example, we are redeclaring spaces to a new value, with a new type. 
        this is valid. If we tried to do the following:

        let mut spaces = "     ";
        spaces = spaces.len();

        We would get a compiler error, because a mut variable cannot have its type changed
    */
    let spaces = "     ";
    let spaces = spaces.len();

    println!("{}", spaces);

}
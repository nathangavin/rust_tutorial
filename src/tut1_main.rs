use std::io; // standard input output
use rand::Rng;
use std::cmp::Ordering;

/*
    fn main() is the entry point of the file
*/
fn main() {
    /*
        println! is a macro (similar but not same to function call)
    */
    println!("Guess the number!");

    /*
        secret_number is of type unsigned 32 bit
        rand::thread_rng() gets a random generator using OS stuff
        gen_range returns a random number from the range provided
        1..=100 is a range of numbers from 1 to 100 inclusive
        the =100 makes it include the 100
    */
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is  {secret_number}");

    /*
        loop {} is an infinite loop, like while(true)
    */
    loop {
        println!("Please input your guess.");
        
        /*
            String::new() is a static constructor for an empty String object
            the mut keyword makes it mutable - variables are immutable by default
            type is String
        */
        let mut guess: String = String::new();

        /*
            io::stdin().read_line(&mut guess) reads the next line input and 
            appends it to the guess variable. passing a reference to the variable
            via the & symbol. arguments in a function are unable to be modified by default,
            using &mut allows for the referenced variable to be edited by the function

            .expect(msg) will throw a panic, ending the program with the message. It catches
            any error that the function returns
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    

        /*
            match is like a switch statement kind of, but for function returns
            the value is the return of guess.trim().parse()

            trim() removes whitespace
            parse() converts string to an int type

            parse() returns a Result Enum<F, F::Err>, so we can match on an OK response, and an Err response
                match can be used to safely handle errors
            to handle each, you can use anon functions/arrow functions
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
     
        println!("You guessed: {guess}");
    
        /*
            <string>.cmp(&<string>) returns an Ordering Enum, which contains <Less, Equal, Greater>
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!"); 
                break;
            },
            Ordering::Greater => println!("Too Big!"),
        }    
    }
    

    

}

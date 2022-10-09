fn main() {
    let number = 7;

    // if statement MUSt be a bool type
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let number2 = if number == 7 { 1 } else { 2 };
    println!("number2 = {}", number2);

    let mut counter = 0;

    /* 
        This code will loop until counter is 10, at which point it will 
        break and return 20, and assign it to the result variable.
    */
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };
    println!("result = {}", result);

    /*
        loops can be named. Note the single ' character by the name
    */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    /*
        conditional while loop
    */
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");


    /*
        looping over an iteratible object
    */
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    /*
        using for loop to loop through range 1..4, which valuates to 1,2,3. we then 
        used rev() to reverse the range, so 3,2,1
    */
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    println!("F to C function");
    let temps = [100.0, 62.5, 32.0];
    for temp in temps {
        println!("Temp {}F = {}C", temp, f_to_c(temp));
    }

    println!("C to F function");
    let temps = [0.0, 10.0, 15.0, 20.0, 25.0, 30.0];
    for temp in temps {
        println!("Temp {}C = {}F", temp, c_to_f(temp));
    }

    println!("Fibonacci n function");
    for a in 1..=10 {
        println!("{}: {}", a, fibonacci_n(a));
    }
}

fn f_to_c(temp: f32) -> f32 {
    ((temp - 32.0) * 5.0) / 9.0
}

fn c_to_f(temp: f32) -> f32 {
    ((temp * 9.0) / 5.0) + 32.0
}

fn fibonacci_n(n: i32) -> i32 {
    let mut last_f = (0, 1);
    
    for a in 2..=n {
        let temp = last_f.1;
        last_f.1 = last_f.0 + temp;
        last_f.0 = temp;
    }
    last_f.1

}
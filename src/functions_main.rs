fn main() {

    // complex expression
    // the block evaluates to 4, which is assigned to y
    // notice the x + 1 line doesnt have a semi colon
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);

    // another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("x = {}", x);
    let x = plus_one(x);
    println!("x = {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

/*
    this function has a return type of i32
    returns can be done implicitly, by leaving the last line
    as an expression (no semicolon)
*/
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
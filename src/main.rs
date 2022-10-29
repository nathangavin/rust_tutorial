
/*
    Structs define the data structure, need to be implemented to define methods
*/
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Colour (i32, i32, i32);
struct Point (i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let mut user1 = User {
        active:true,
        username: String::from("nathangavin"),
        email: String::from("nathangavin987@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("nathan.gavin@ideqa.com");

    let user2 = build_user(String::from("test@test.com"), String::from("test"));

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("test2@test.com"),
        sign_in_count: user1.sign_in_count
    };

    let user4 = User {
        email: String::from("test3@test.com"),
        ..user3
    };

    let black = Colour(0,0,0);
    let origin = Point(0,0,0);

    let rect = Rectangle {
        width: 10,
        height: 20
    };

    println!("The area is {}", area(&rect));
    println!("The rect is {:#?}", rect);



}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn area(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
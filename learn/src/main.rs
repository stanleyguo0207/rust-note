fn main() {
    println!("Hello, world!");
}

#[test]
fn test_expr() {
    let x = {
        let y = 5;
        y + 1
    };

    println!("{}", x)
}

#[test]
fn test_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if 10 == counter {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}

#[test]
fn test_loop2() {
    let mut count = 0;

    'coutting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if 9 == remaining {
                break;
            }

            if 2 == count {
                break 'coutting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

#[test]
fn test_loop3() {
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!")
}

#[test]
fn test_string() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

#[test]
fn test_move() {
    let s1 = String::from("hello");
    // let s2 = s1;

    println!("{}, world!", s1);
}

#[test]
fn test_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);
}

#[test]
fn test_clone2() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn test_first_word() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word)
}

#[test]
fn test_first_word2() {
    let s = "hello world";

    let word = first_word(s);

    println!("the first word is: {}", word)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[test]
fn test_struct() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[test]
fn test_match() {
    println!("coin is {}", value_in_cents(Coin::Dime));

    println!(
        "coin is {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    )
}

#[test]
fn test_if_let() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

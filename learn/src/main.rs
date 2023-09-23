use std::fmt::Display;

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

#[test]
fn test_vector() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(12);
    v.push(13);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}

#[test]
fn test_string2() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // println!("s1 is {s1}");
    println!("s1 is {s2}");
    println!("s1 is {s3}");
}

#[test]
fn test_string3() {
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

#[test]
fn test_hash_map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

#[test]
fn test_hash_map2() {
    use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

#[test]
fn test_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[test]
fn test_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x = {}, y = {}", self.x, self.y);
        } else {
            println!("y = {}, x = {}", self.y, self.x);
        }
    }
}

#[test]
fn test_point() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2: Point<f32> = Point { x: 10.0, y: 5.0 };
    println!("distace = {}", p2.distance_from_origin());

    p.cmp_display();
    p2.cmp_display();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_longest_with_an_announcement() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

fn generate_workout(intensity: u32, random_number: u32) {
    use std::thread;
    use std::time::Duration;

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

#[test]
fn test_generate_workout() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn test_lambda1() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

#[test]
fn test_lambda2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[test]
fn test_lambda3() {
    use std::thread;

    let list = vec![1, 2, 3];
    println!(
        "[{:?}] Before defining closure: {:?}",
        thread::current().id(),
        list
    );

    thread::spawn(move || println!("[{:?}] From thread: {:?}", thread::current().id(), list))
        .join()
        .unwrap();
}

#[test]
fn test_sort_by_key() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[test]
fn test_summary() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

pub fn summary_notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

#[test]
fn test_summary2() {
    let news_article = NewsArticle {
        headline: String::from("news"),
        location: String::from("shanghai"),
        author: String::from("tommy"),
        content: String::from("news 123"),
    };
    summary_notify(&news_article)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[test]
fn test_inventory() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[test]
fn test_iterator1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn test_iterator2() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn test_iterator3() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn test_filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    )
}

#[test]
fn test_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[test]
fn test_list() {
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

#[test]
fn test_box_addr() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn mybox_hello(name: &str) {
    println!("Hello, {name}!");
}

#[test]
fn test_mybox_cast() {
    let m = MyBox::new(String::from("Rust"));
    mybox_hello(&m);
    // mybox_hello(&(*m)[..]);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn test_custom_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

use std::rc::Rc;

enum ListRef {
    Cons2(i32, Rc<ListRef>),
    Nil2,
}

#[test]
fn test_list_ref() {
    use ListRef::{Cons2, Nil2};

    let a = Rc::new(Cons2(
        1,
        Rc::new(Cons2(2, Rc::new(Cons2(3, Rc::new(Nil2))))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

use std::cell::RefCell;

#[derive(Debug)]
enum ListRcRef {
    Cons(Rc<RefCell<i32>>, Rc<ListRcRef>),
    Nil,
}

#[test]
fn test_list_rcref() {
    use crate::ListRcRef::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum ListRefLoop {
    Cons(i32, RefCell<Rc<ListRefLoop>>),
    Nil,
}

impl ListRefLoop {
    fn tail(&self) -> Option<&RefCell<Rc<ListRefLoop>>> {
        match self {
            ListRefLoop::Cons(_, item) => Some(item),
            ListRefLoop::Nil => None,
        }
    }
}

#[test]
fn test_list_ref_loop() {
    use crate::ListRefLoop::{Cons, Nil};

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

use std::thread;
use std::time::Duration;

#[test]
fn test_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

#[test]
fn test_spawn_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vertor: {:?}", v);
    });
    handle.join().unwrap();
}

use std::sync::mpsc;

#[test]
fn test_spawn_mpsc_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val: {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[test]
fn test_spawn_mpsc_channel2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn test_spawn_mpsc_channel3() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

use std::sync::Mutex;

#[test]
fn test_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

use std::sync::Arc;

#[test]
fn test_mutex2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

#[test]
fn test_unsafe1() {
    let mut num = 5;
    let ptr1 = &num as *const i32;
    let ptr2 = &mut num as *mut i32;

    unsafe {
        println!("*ptr1 = {}", *ptr1);
        println!("*ptr2 = {}", *ptr2);
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PrintPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for PrintPoint {}

impl fmt::Display for PrintPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[test]
fn test_outline_print() {
    let p = PrintPoint { x: 1, y: 3 };
    p.outline_print();
}

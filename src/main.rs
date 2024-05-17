#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(non_snake_case)]

mod macros;
mod player;
mod sumofsquares;
mod traits;
mod operatorOverloading;
mod practice;
mod files;
mod errorhandling;
mod threadinMP;
mod concurrencychannels;

use crate::archive::arch::archive_file;
mod archive;

use crate::Suit::{Clubs, Diamonds, Hearts, Spades};

pub fn print<T: std::fmt::Display>(a: T) {
    println!("{}", a);
}

static mut COUNT: i32 = 0;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

fn getOranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount < 0) => "invalid number of",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "a lots of",
    };
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Hearts => {
            println!("\u{2665}"); // Heart
        }
        Suit::Diamonds => {
            println!("\u{2660}"); // Diamond
        }
        Suit::Clubs => {
            println!("\u{2663}"); // Club
        }
        Suit::Spades => {
            println!("\u{2666}"); // Spade
        }
    }
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: i32,
    salary: f64,
}
impl Employee {
    fn fn_details(&self) -> String {
        format!(
            "{} is {} years old and earns {}",
            self.name, self.age, self.salary
        )
    }

    fn static_fn_details() -> String {
        format!("Static function")
    }
}
fn main() {
    for i in -1..15 {
        println!("{}. i Have {} oranges", i, getOranges(i));
    }
    {
        let a = 3;
    }

    unsafe {
        COUNT += 1;
    }

    // println!("{}", a);

    print_choice(Hearts);
    print_choice(Clubs);
    print_choice(Diamonds);
    print_choice(Spades);
    let emp = Employee {
        name: String::from("Narayan"),
        age: 32,
        salary: 1000.0,
    };
    println!("{:?}", emp);
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn_details());

    let color = Colors::Red;

    const URL: &str = "www.google.com";
    /*
    upper case by convention
    data type ismandatory
    shadowing is not permited
    global or local space
     */

    let mut name = "Narayan";
    let age = 32;
    let money = 12.3;
    let amount: i64 = 2432423424234234324;
    name = "Hell oworld";
    let age = "hello";
    let (a, b, c) = (3, 4, "red");
    let million = 1_000_000;
    println!("{}", million);
    let is_day = true;
    let is_night = false;
    print!("{}", is_day);
    let char1: char = 'a';

    let cat = "Fluffy";

    let cat: &'static str = "Fluffy";

    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    let numbers = [0; 15];
    println!("{}", primes[0]);
    println!("{:?}", primes);
    println!("{:?}", numbers);

    let primes: Vec<i32> = Vec::new();
    let mut primes: Vec<i32> = vec![2, 3, 5, 7, 11];
    print!("{:?}", primes);
    primes.push(13);
    print!("{:?}", primes);
    primes.remove(2);
    print!("{:?}", primes);

    for number in numbers.iter() {
        print(number);
    }
    let my_num = 5;
    if my_num > 1 {
        print!("greater than 1");
    }

    let dog = String::new();
    let mut dog = String::from("Rusty");
    print(&dog);
    print(&dog.len());
    dog.push_str("The dog");
    print(&dog);

    print(a + b);
    print(a * b);
    print(a / b);
    print(a - b);
    print(a % b);
    /*
    ++ and -- i.e. increment and decrement operators are not available in rust
     */

    player::play_movie("The Matrix");
    player::play_song("TheMatrix.mp3");

    clean::clean_house();
    clean::files::clean_files();

    archive_file("somefile.txt");

    /*
    A slice is a pointer to a block of memory
    can be used on array vector and string
    indexed
    size is determined at runtime
     */

    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    let my_tuples = (2, "apple", 50.0, false); // used to pass around heterogonous data
    println!("{}", my_tuples.1);
    let (count, fruit, price, available) = my_tuples;

    let mut my_name = "Narayan";

    say_hi(&mut my_name);

    let sum = |a: i32, b: i32| -> i32 { a + b };
    sum(3, 4);

    let a = |a: i32| a + 1;
    println!("{}", a(3));
    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };
    print(b(3));

    let square = |a: i32| a * a;

    apply(square, 3);
}

// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }

mod clean {
    pub fn clean_house() {
        println!("Cleaning house");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Cleaning files");
        }
    }
}

fn say_hi(name: &mut &str) {
    *name = "Poudel";
    println!("Hi {}", name);
}

/*
THe HOF , higher order functions are the functions that take other functions as arguments or return functions as results.

*/

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}

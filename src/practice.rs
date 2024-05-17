#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use std::io::{self, Write};

fn main() {
    // println!("{}", "Hello world");
    // arith(2, 3, '/');
    // println!("{}", farenheit_to_celcius(98.0));
    // println!("{}", check_prime(5));
    // print_fibonacci(10);
    // reverse_string(String::from("hello"));
    // count_vowel(String::from("aeiouuusdsd"));
    // sum_of_digit(12345);
    // println!("{}", factorial(3));
    address_book();
}

fn arith(a: i32, b: i32, symbol: char) {
    match symbol {
        symbol if symbol == '+' => println!("The sum is {}", a + b),
        symbol if symbol == '-' => println!("The subtraction is {}", a - b),
        symbol if symbol == '*' => println!("The multiplication is {}", a * b),
        symbol if symbol == '/' => println!("The division is {:.3}", a as f32 / b as f32),
        _ => println!("Some thing went wrong"),
    }
}

fn farenheit_to_celcius(farenheit_value: f32) -> f32 {
    (farenheit_value - 32.0) * (5.0 / 9.0)
}

fn check_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let sqrt_limit = (num as f64).sqrt() as i32 + 1;
    for i in (3..sqrt_limit).step_by(2) {
        if num % i == 0 {
            false;
        }
    }
    true
}

fn print_fibonacci(upto: i32) {
    if upto <= 0 {
        println!("Please enter a positive integer.");
        return;
    }
    let (mut prev, mut curr) = (0, 1);
    let mut next;
    println!("{}", prev);
    if upto == 1 {
        return;
    }
    println!("{}", curr);
    for _ in 2..upto {
        next = prev + curr;
        println!("{}", next);
        (prev, curr) = (curr, prev);
    }
}

fn reverse_string(string: String) {
    let mut reverse_string = String::with_capacity(string.len());

    for character in string.chars() {
        reverse_string.push(character);
    }
    println!("{}", reverse_string);

    // or simple code
    let mut reverse_string2 = String::with_capacity(string.len());

    reverse_string2 = string.chars().rev().collect();
    println!("{}", reverse_string2);
}

fn count_vowel(string: String) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count: i32 = 0;
    for character in string.chars() {
        if vowels.contains(&character) {
            count += 1;
        }
    }
    println!("no of vowels: {}", count);
}

fn sum_of_digit(mut num: i64) {
    let mut sum = 0;
    let mut digit = 0;
    while num > 0 {
        digit = num % 10;
        num = num / 10;
        sum += digit;
    }
    println!("{}", sum);
}

fn factorial(num: i32) -> i32 {
    if num <= 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}

struct Contact {
    name: String,
    phone_number: String,
}

impl Contact {
    fn new(name: String, phone_number: String) -> Contact {
        Contact { name, phone_number }
    }
}

fn address_book() {
    let mut contacts: Vec<Contact> = Vec::new();

    println!("Your Personal Contact Management Software");
    loop {
        println!("Make your choice: ");
        println!("1. Add contact");
        println!("2. View Contact");
        println!("3. Remove Contact");
        println!("4 . Exit");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();
        match choice {
            1 => add_contact(&mut contacts),
            2 => view_contact(&contacts),
            3 => println!("Removing phone number"),
            4 => {
                println!("BYE! have a good day!!");
                break;
            }
            _ => println!("Invalid number , please enter again!"),
        }
    }
}

fn add_contact(contacts: &mut Vec<Contact>) {
    println!("Enter name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter Phone Number: ");
    io::stdout().flush().unwrap();
    let mut phone_number = String::new();
    io::stdin().read_line(&mut phone_number).unwrap();
    

    contacts.push(Contact::new(
        name.trim().to_string(),
        phone_number.trim().to_string(),
    ));

    println!("Contact added");
}

fn view_contact(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("No contacts added ");
        return;
    }
    println!("\tContacts: ");
    for contact in contacts {
        println!(
            "\t Name : {} \t Phone Number: {}",
            contact.name, contact.phone_number
        );
    }
}

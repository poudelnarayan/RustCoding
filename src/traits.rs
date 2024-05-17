#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
/*
TRAITS:
similar to interfaces or abstract classes in other languages
add a definition to a structure
can have definition only or a default implementation
can have instance and non - instance action

*/

struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        println!("{}", "Dog is barking");
        return format!("{} is barking", self.species);
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark());
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello, I am a developer");
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "RUST"
    }

    fn say_hello(&self) {
        println!("println!('Hello, I am a Rust Developer')");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome }
    }

    fn language(&self) -> &str {
        "JAVA"
    }

    fn say_hello(&self) {
        println!("System.out.println('Hello, I am a Java Developer')");
    }
}

fn main() {
    let r = RustDev::new(true);
    let r1 = RustDev { awesome: true };

    let j = JavaDev::new(false);

    let dog = Dog {
        species: "retriever",
    };
    let cat = Cat { color: "black" };

    bark_it(dog);

    // println!("{}", r.language());
    // r.say_hello();
    // println!("{}", j.language());
    // j.say_hello();
}

/*
Macros:
write code that write code - META PROGRAMMING
Match an expression and perform some operation
code is explained and compiled

for example : printline!() is a macro
also format!() is a macro
macros has exclamation signs at the end

*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

macro_rules! name {
    ($name: expr) => {
        println!("Hello, {}", $name)
    };
}

macro_rules! sayHello{
    ($($name:expr),*) => (
        $(println!("Hello, {}", $name);)*
    );
}

fn main() {
    my_macro!();
    name!("Rustacean");
    sayHello! ("john", "Narayan", "Alex");
}

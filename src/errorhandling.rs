#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{
    fs::File,
    io::{self, Read},
};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => {
//             return Err(e);
//         }
//     };
//     let mut a = String::new();
//     match f.read_to_string(&mut a) {
//         Ok(_) => Ok(a),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error>{
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)


}

fn main() {
    // let v = vec![1, 2, 3];
    // v[10];

    // let f = File::open("main.jpg");
    // match f {
    //     Ok(f) => {
    //         println!("File found {:?}", f);
    //     }
    //     Err(e) => {
    //         println!("file not found \n {:?}", e);
    //     }
    // }
    // println!("Continuing on the execution");

    // divide(Some(1));
    // divide(Some(0));
    // divide(None);

    let a = read_username_from_file();
    println!("{:?}", a);
}

const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Cannot divide by 0"),
        Some(x) => println!("result is{}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
    }
}

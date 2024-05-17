#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{
    fs::{remove_file, File, OpenOptions},
    io::{Read, Write},
};

fn main() {
    // let mut file = File::create("example.txt").expect("create failed");
    // file.write_all("Hello World!\n".as_bytes()).expect("Write failed");

    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .open("example.txt")
    //     .expect("cannot open");
    // file.write_all("Adding content to the file\n".as_bytes())
    //     .expect("Cannot read file");

    // let mut file = std::fs::File::open("example.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // println!("{}",contents);

    remove_file("example.txt").expect("delete failed");
}

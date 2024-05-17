#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{thread::{self, sleep, JoinHandle}, time::Duration};

fn main() {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i*1000));
            println!("New thread {} ", i);
        });
        threads.push(th);
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("Main Thread");
}

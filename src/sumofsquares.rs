fn main() {
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!(
        "The sum of squares which are less than 500 and are even is : {}",
        sum
    );

    /*
    USING HOFS( higher order functions)
     */

    let sum2 = (0..)
        .map(|a: i32| a * a)
        .take_while(|&a| a < limit)
        .filter(|&a| is_even(a))
        .fold(0, |sum3, a| sum3 + a);

    println!("The sum using HOFS : {}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

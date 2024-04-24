use std::{time::Instant, vec};

mod arr;
mod stack;
mod two_pointer;

fn main() {
    let now = Instant::now();
    let input = vec![2, 7, 11, 15];
    let output = two_pointer::two_sum(input, 9);
    println!("The output is {:?}", output);

    let elapsed = now.elapsed();
    println!("The time diff is {:?}", elapsed);
}

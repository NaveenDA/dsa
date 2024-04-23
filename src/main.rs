use std::time::Instant;

mod arr;
mod stack;
fn main() {
   let now  = Instant::now();
   let elapsed = now.elapsed();
   println!("The time diff is {:?}", elapsed);
}

use std::time::Instant;
// use crate::day1::solve_day1;

mod day1;

fn main() {
    let now = Instant::now();

    // solve_day1();

    println!("\nSolving problem took {:?}ms", now.elapsed().as_millis());
}

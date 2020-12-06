use std::time::Instant;
// use crate::day1::solve_day1;
// use crate::day2::solve_day2;

mod day1;
mod day2;

fn main() {
    let now = Instant::now();

    // solve_day1();
    // solve_day2();

    println!("\nSolving problem took {:?}ms", now.elapsed().as_millis());
}

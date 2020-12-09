use std::time::Instant;
// use crate::day1::solve_day1;
// use crate::day2::solve_day2;
// use crate::day3::solve_day3;
// use crate::day4::solve_day4;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let now = Instant::now();

    // solve_day1();
    // solve_day2();
    // solve_day3();
    // solve_day4();

    println!("\nSolving problem took {:?}ms", now.elapsed().as_millis());
}

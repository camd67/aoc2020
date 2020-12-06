/// https://adventofcode.com/2020/day/1
/// Finds two entires in the data set that sum to 2020. Returns the result of multiplying those two
#[allow(dead_code)]
pub fn solve_day1() {
    let raw_expense_report: &str = include_str!("./data/day1.txt");

    // Gather up our expenses
    let mut expenses: Vec<u32> = Vec::new();
    for line in raw_expense_report.lines() {
        match line.parse::<u32>() {
            Ok(num) => expenses.push(num),
            Err(e) => println!("{:?} Could not parse number", e),
        }
    }

    part1(&expenses);
    part2(&expenses);
}

/// Find our sum to 2020, and multiply them
/// There's probably some way to sort these to get a better big-o but this works like, plenty fast
fn part1(expenses: &[u32]) {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                println!("Two expenses: {}", expenses[i] * expenses[j]);
                return
            }
        }
    }
}

/// Find the three expenses that sum to 2020 and multiply them.
/// Probably a better way!
fn part2(expenses: &[u32]) {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            for k in j..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    println!("Three expenses: {}", expenses[i] * expenses[j] * expenses[k]);
                    return
                }
            }
        }
    }
}

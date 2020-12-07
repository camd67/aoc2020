use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Pos(usize, usize);

const TREE_CHAR: char = '#';

#[allow(dead_code)]
pub fn solve_day3() {
    let raw_slope = include_str!("./data/day3.txt");

    let mut trees: HashSet<Pos> = HashSet::new();

    let mut height = 0;
    let mut width = 0;

    // Build up our set of trees
    for (y, line) in raw_slope.lines().enumerate() {
        for (x, input_char) in line.chars().enumerate() {
            if input_char == TREE_CHAR {
                trees.insert(Pos(x, y));
            }
            // Little tricky here, but we'll use this to always update our width variable.
            // It'll settle out on the actual width after the final iteration.
            width = x;
        }
        height = y;
    }
    println!("Found {} trees", trees.len());
    println!("Scanning a {} x {} board", width, height);

    let tree_collisions = [
        check_for_trees(1, 1, height, width, &trees),
        check_for_trees(3, 1, height, width, &trees),
        check_for_trees(5, 1, height, width, &trees),
        check_for_trees(7, 1, height, width, &trees),
        check_for_trees(1, 2, height, width, &trees),
    ];

    println!("Part 1: {}", tree_collisions[1]);

    let tree_product: u32 = tree_collisions.iter().product();
    println!("Multiplying all tree collisions: {}", tree_product)
}

fn check_for_trees(
    slope_x: usize,
    slope_y: usize,
    height: usize,
    width: usize,
    trees: &HashSet<Pos>,
) -> u32 {
    let mut tree_hit_count = 0;
    let mut x = 0;

    for y in (0..=height).step_by(slope_y) {
        let pos_to_check = Pos(x % (width + 1), y);
        if trees.contains(&pos_to_check) {
            // println!("Hit {:?}", pos_to_check);
            tree_hit_count += 1;
        }
        x += slope_x;
    }

    tree_hit_count
}

use std::fs;

mod challenges;

fn main() {
    let contents = fs::read_to_string("data/day_2.txt").expect("Should have been able to read file");

    let counts = challenges::day_02::group_sums_part_two(contents.lines().collect());

    let sum: i32 = counts.iter().sum();

    println!("sum {sum}")
}


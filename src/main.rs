use std::fs;

mod challenges;

fn main() {
    let contents =
        fs::read_to_string("data/day_05.txt").expect("Should have been able to read file");

    let output = challenges::day_05::apply_all_instructions(contents.lines().collect());

    println!("top containers {output}")
}

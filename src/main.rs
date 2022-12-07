use std::fs;

mod challenges;

fn main() {
    let contents: String =
        fs::read_to_string("data/day_07.txt").expect("Should have been able to read file");

    let output = challenges::day_07::assembly(contents.lines().collect());

    println!("sum {}", output)
}

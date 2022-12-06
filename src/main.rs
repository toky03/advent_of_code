use std::fs;

mod challenges;

fn main() {
    let contents: String =
        fs::read_to_string("data/day_06.txt").expect("Should have been able to read file");

    let output = challenges::day_06::find_first_distinct_sequence(contents, 14);

    println!("top containers {}", output.unwrap())
}

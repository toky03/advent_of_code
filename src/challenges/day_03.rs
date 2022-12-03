fn split_half(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_doublette(compartements: (&str, &str)) -> Option<char> {
    let (left, right) = compartements;
    for c in left.chars() {
        if right.contains(c) {
            return Some(c);
        }
    }
    return None;
}

fn find_grouped_doublette(group: Vec<&str>) -> Option<char> {
    let bag_1 = group.get(0).unwrap().chars();
    let bag_2 = group.get(1).unwrap().to_owned();
    let bag_3 = group.get(2).unwrap().to_owned();
    let intersection: Vec<char> = bag_1
        .filter(|char_bag_one| bag_2.contains(*char_bag_one) && bag_3.contains(*char_bag_one))
        .collect();
    intersection.get(0).map(|c| c.to_owned())
}

fn group_into_three_packages(input_lines: Vec<&str>) -> Vec<Vec<&str>> {
    let mut output: Vec<Vec<&str>> = Vec::new();
    let mut input_copy: Vec<&str> = input_lines.clone();
    input_copy.reverse();
    for _ in 0..(input_lines.len() / 3) {
        output.push(vec![
            input_copy.pop().unwrap(),
            input_copy.pop().unwrap(),
            input_copy.pop().unwrap(),
        ])
    }
    output
}

fn evaluate_priority(letter: char) -> Option<i32> {
    let asci_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    asci_letters
        .to_owned()
        .chars()
        .position(|a| a == letter)
        .map(|position| position as i32 + 1)
}

pub fn group_sums(input_lines: Vec<&str>) -> Vec<i32> {
    input_lines
        .into_iter()
        .map(|line| {
            let splitted = split_half(line);
            let doublette = find_doublette(splitted).unwrap();
            evaluate_priority(doublette).unwrap()
        })
        .collect()
}

pub fn group_sums_part_two(input_lines: Vec<&str>) -> Vec<i32> {
    let grouped_parts = group_into_three_packages(input_lines);
    grouped_parts
        .into_iter()
        .map(|line| {
            let doublette = find_grouped_doublette(line);
            evaluate_priority(doublette.unwrap()).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::evaluate_priority;
    use super::find_doublette;
    use super::find_grouped_doublette;
    use super::group_into_three_packages;
    use super::group_sums;
    use super::split_half;

    #[test]
    fn test_split() {
        assert_eq!(
            split_half("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        )
    }

    #[test]
    fn test_find_doublette() {
        assert_eq!(find_doublette(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), Some('p'));
    }

    #[test]
    fn test_priority() {
        assert_eq!(evaluate_priority('p'), Some(16));
        assert_eq!(evaluate_priority('L'), Some(38));
        assert_eq!(evaluate_priority('P'), Some(42));
        assert_eq!(evaluate_priority('v'), Some(22));
        assert_eq!(evaluate_priority('t'), Some(20));
        assert_eq!(evaluate_priority('s'), Some(19));
    }

    #[test]
    fn test_group_part_one() {
        assert_eq!(
            group_sums(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]),
            vec![16, 38, 42, 22, 20, 19]
        )
    }

    #[test]
    fn test_grouped_part_two() {
        assert_eq!(
            find_grouped_doublette(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ]),
            Some('r')
        );
        assert_eq!(
            find_grouped_doublette(vec![
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]),
            Some('Z')
        );
    }

    #[test]
    fn test_group_into_three() {
        assert_eq!(
            group_into_three_packages(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]),
            vec![
                vec![
                    "vJrwpWtwJgWrhcsFMMfFFhFp",
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                    "PmmdzqPrVvPwwTWBwg"
                ],
                vec![
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                    "ttgJtRGJQctTZtZT",
                    "CrZsJsPPZsGzwwsLwLmpwMDw"
                ]
            ]
        )
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            group_sums(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ])
            .iter()
            .sum::<i32>(),
            157
        )
    }
}

use std::collections::HashSet;

fn split_to_lists(input_lines: Vec<&str>) -> Vec<(Vec<i32>, Vec<i32>)> {
    input_lines
        .into_iter()
        .map(|line| split_line(line))
        .collect()
}

fn split_line(input_line: &str) -> (Vec<i32>, Vec<i32>) {
    let mut splitted = input_line.split(",");
    let (left, right) = (splitted.next().unwrap(), splitted.next().unwrap());
    let mut left_splitted = left.split("-");
    let mut right_splitted = right.split("-");

    let (left_start, left_end) = (
        left_splitted.next().unwrap().parse::<i32>().unwrap(),
        left_splitted.next().unwrap().parse::<i32>().unwrap(),
    );

    let (right_start, right_end) = (
        right_splitted.next().unwrap().parse::<i32>().unwrap(),
        right_splitted.next().unwrap().parse::<i32>().unwrap(),
    );
    (
        (left_start..left_end + 1).collect(),
        (right_start..right_end + 1).collect(),
    )
}

fn is_sublist(left: Vec<i32>, right: Vec<i32>) -> bool {
    left.iter().all(|item| right.contains(item)) || right.iter().all(|item| left.contains(item))
}

fn is_overlapping(left: Vec<i32>, right: Vec<i32>) -> bool {
    let left_set: HashSet<i32> = HashSet::from_iter(left.into_iter());
    let right_set: HashSet<i32> = HashSet::from_iter(right.into_iter());

    let intersection = left_set.intersection(&right_set);

    intersection.into_iter().collect::<Vec<&i32>>().len() > 0
}

pub fn group_sublists(input_lines: Vec<&str>) -> Vec<i32> {
    split_to_lists(input_lines)
        .into_iter()
        .map(|(left, right)| {
            if is_overlapping(left, right) {
                return 1;
            }
            0
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::is_sublist;
    use super::split_line;

    #[test]
    fn test_create_lists() {
        assert_eq!(split_line("2-4,6-8"), (vec![2, 3, 4], vec![6, 7, 8]));
    }

    #[test]
    fn test_contains() {
        assert_eq!(
            is_sublist(vec![2, 3, 4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7]),
            true
        );
        assert_eq!(is_sublist(vec![6], vec![4, 5, 6]), true);
        assert_eq!(is_sublist(vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7, 8]), false);
    }
}

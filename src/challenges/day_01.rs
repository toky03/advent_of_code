pub fn group_sums(input_lines: Vec<&str>) -> Vec<i32> {
    input_lines.into_iter().fold(vec![0], |mut prev, curr| {
        if !curr.is_empty() {
            let counter = prev.pop().unwrap() + curr.parse::<i32>().unwrap();
            prev.push(counter);
        } else {
            prev.push(0);
        }
        prev
    })
}

pub fn top_tree_elfs(mut counts: Vec<i32>) -> i32 {
    counts.sort();
    let max = counts.get(counts.len() - 1).unwrap();
    let second = counts.get(counts.len() - 2).unwrap();
    let third = counts.get(counts.len() - 3).unwrap();
    max + second + third
}

#[cfg(test)]
mod tests {
    use super::group_sums;
    use super::top_tree_elfs;

    #[test]
    fn test_grouping() {
        assert_eq!(group_sums(vec!["1", "2", "", "2", "4"]), vec![3, 6]);
    }

    #[test]
    fn test_top_tree() {
        assert_eq!(top_tree_elfs(vec![5, 1, 7, 8, 2]), 20);
    }
}

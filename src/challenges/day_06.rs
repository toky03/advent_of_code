use std::collections::HashSet;

fn is_distinct(sequence: &str) -> bool {
    let mut uniq = HashSet::new();
    sequence.chars().into_iter().all(move |x| uniq.insert(x))
}

pub fn find_first_distinct_sequence(
    sequence: String,
    required_sequence_length: usize,
) -> Option<i32> {
    for (i, elem) in sequence.chars().enumerate() {
        if i + required_sequence_length > sequence.len() {
            return None;
        }
        if is_distinct(&sequence[i..i + required_sequence_length]) {
            return Some((i + required_sequence_length) as i32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::find_first_distinct_sequence;
    use super::is_distinct;

    #[test]
    fn test_is_distinct() {
        assert_eq!(is_distinct("abcdef"), true);
        assert_eq!(is_distinct("abcdea"), false);
    }

    #[test]
    fn find_start_index_first_distinct_part_one() {
        let part_one_sequence_length = 4;
        assert_eq!(
            find_first_distinct_sequence(
                String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                part_one_sequence_length
            ),
            Some(5)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                part_one_sequence_length
            ),
            Some(6)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                part_one_sequence_length
            ),
            Some(10)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
                part_one_sequence_length
            ),
            Some(11)
        );
    }

    #[test]
    fn find_start_index_first_distinct_part_two() {
        let part_two_sequence_length = 14;
        assert_eq!(
            find_first_distinct_sequence(
                String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
                part_two_sequence_length
            ),
            Some(19)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                part_two_sequence_length
            ),
            Some(23)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                part_two_sequence_length
            ),
            Some(23)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                part_two_sequence_length
            ),
            Some(29)
        );
        assert_eq!(
            find_first_distinct_sequence(
                String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
                part_two_sequence_length
            ),
            Some(26)
        );
    }
}

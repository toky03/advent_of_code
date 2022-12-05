use std::collections::HashMap;

enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    pub fn outcome(&self, oponent: &Self) -> i32 {
        match self {
            Move::Rock => match oponent {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissor => 6,
            },
            Move::Paper => match oponent {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissor => 0,
            },
            Move::Scissor => match oponent {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissor => 3,
            },
        }
    }

    fn from(code: &str) -> Self {
        if code == "X" || code == "A" {
            return Move::Rock;
        }
        if code == "Y" || code == "B" {
            return Move::Paper;
        }
        return Move::Scissor;
    }

    fn to_win(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }

    fn to_loose(&self) -> Self {
        match self {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        }
    }

    fn to_draw(&self) -> Self {
        match self {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissor => Move::Scissor,
        }
    }

    fn move_value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
}

pub fn group_sums(input_lines: Vec<&str>) -> Vec<i32> {
    input_lines
        .into_iter()
        .map(|line| {
            let mut splitted = line.split_whitespace();
            let oponent = splitted.next().unwrap();
            let own_move = splitted.next().unwrap();
            evaluate_move(oponent, own_move)
        })
        .collect()
}

fn evaluate_move(oponent: &str, own: &str) -> i32 {
    let own_move_value_map: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let move_points = own_move_value_map.get(own).unwrap();
    let win_points = Move::from(own).outcome(&Move::from(oponent));
    move_points + win_points
}

pub fn group_sums_part_two(input_lines: Vec<&str>) -> Vec<i32> {
    input_lines
        .into_iter()
        .map(|line| {
            let mut splitted = line.split_whitespace();
            let oponent = splitted.next().unwrap();
            let own_move = splitted.next().unwrap();
            evaluate_what_to_choose(oponent, own_move)
        })
        .collect()
}

fn evaluate_what_to_choose(oponent: &str, strategy: &str) -> i32 {
    let own_move_fn_map: HashMap<&str, fn(&Move) -> Move> = HashMap::from([
        ("X", Move::to_loose as fn(&Move) -> Move),
        ("Y", Move::to_draw as fn(&Move) -> Move),
        ("Z", Move::to_win as fn(&Move) -> Move),
    ]);
    let own_move = own_move_fn_map.get(strategy).unwrap()(&Move::from(oponent));
    let win_points = own_move.outcome(&Move::from(oponent));
    own_move.move_value() + win_points
}

#[cfg(test)]
mod tests {
    use super::group_sums;
    use super::group_sums_part_two;
    use super::Move;

    #[test]
    fn test_moves_outcome() {
        assert_eq!(Move::Rock.outcome(&Move::Rock), 3);
        assert_eq!(Move::Paper.outcome(&Move::Paper), 3);
        assert_eq!(Move::Scissor.outcome(&Move::Scissor), 3);
        assert_eq!(Move::Rock.outcome(&Move::Paper), 0);
        assert_eq!(Move::Rock.outcome(&Move::Scissor), 6);
        assert_eq!(Move::Paper.outcome(&Move::Rock), 6);
        assert_eq!(Move::Paper.outcome(&Move::Scissor), 0);
        assert_eq!(Move::Scissor.outcome(&Move::Rock), 0);
        assert_eq!(Move::Scissor.outcome(&Move::Paper), 6);
    }

    #[test]
    fn test_outcome() {
        assert_eq!(group_sums(vec!["A Y", "B X", "C Z"]), vec![8, 1, 6])
    }

    #[test]
    fn test_evaluate_what_to_choose() {
        assert_eq!(
            group_sums_part_two(vec!["A Y", "B X", "C Z"]),
            vec![4, 1, 7]
        )
    }
}

use ndarray::{array, Array, Axis, Ix2, s, ShapeBuilder};

fn parse_input(input: &str) -> Array<i32, Ix2> {
    let len = input.lines().collect::<Vec<&str>>().len();
    let v = input
        .lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        }).reduce(|mut l:  Vec<i32>, r: Vec<i32>| {
        let mut r_cloned = r.clone();
        l.append(&mut r_cloned);
        l
    }).unwrap();

    Array::from_shape_vec((len, len), v).unwrap()
}

fn check_greater(mut input: Array<i32, Ix2>) -> Array<i32, Ix2 > {
    let mut sizes = input.shape();
    let len_x = sizes[0];
    let len_y = sizes[1];
    for x in 0..len_x {
        let mut max: i32 =  -1;
        for y in 0..len_y {
            if input[[x, y]].gt(&max) {
                max = input[[x,y]];
                input[[x, y]] = 1;
            } else {
                input[[x, y]] = 0;
            }
        }
    }
    input
}

fn rotate_90(input: Array<i32, Ix2>) -> Array<i32, Ix2> {
    let copy = input.reversed_axes();
    let reversed = copy.slice_move(s![..,..;-1]);
    reversed
}

fn part_one(input: &str) -> u32 {
    let vec = parse_input(input);
    let original = vec.clone();
    let rotate_1 = rotate_90(original.clone());
    let rotate_2 = rotate_90(rotate_1.clone());
    let rotate_3 = rotate_90(rotate_2.clone());

    let original_checked = check_greater(original);
    let rotate_1_checked = check_greater(rotate_1);
    let rotate_2_checked = check_greater(rotate_2);
    let rotate_3_checked = check_greater(rotate_3);

    println!("original {:?}", original_checked);
    println!("1 {:?}", rotate_1_checked);
    println!("2 {:?}", rotate_2_checked);
    println!("3 {:?}", rotate_3_checked);

    let result = (original_checked | rotate_1_checked)| (rotate_2_checked | rotate_3_checked);
    println!("Result {:?}", result);
    result.sum().try_into().unwrap()

}

#[cfg(test)]
mod tests {
    use ndarray::array;
    use super::parse_input;
    use super::check_greater;
    use super::rotate_90;
    use super::part_one;

    #[test]
    fn test_parse_grid() {
        let input = "30373
25512
65332
33549
35390";

        let expected = array![
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ];

        let result = parse_input(input);

        assert_eq!(result, expected);

    }

    #[test]
    fn test_greater_check() {
        let input = array![
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ];

        let expected = array![
            [1, 0, 0, 1, 0],
            [1, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 1, 0],
        ];


        let indicator_array_ = check_greater(input);

        assert_eq!(indicator_array_, expected);

    }

    #[test]
    fn test_rotation() {
        let input = array![
            [3, 0, 1],
            [2, 5, 1],
            [5, 5, 9],
        ];
        let extected = array![
            [5, 2, 3],
            [5, 5, 0],
            [9, 1, 1],
        ];

        let result = rotate_90(input);
        assert_eq!(result, extected);

    }

    #[test]
    fn test_overlap() {
        let left = array![
            [1, 0, 1],
            [0, 0, 0],
            [0, 0, 0],
        ];
        let right = array![
            [0, 0, 0],
            [0, 0, 0],
            [1, 0, 1],];

        let expected = array![
            [1, 0, 1],
            [0, 0, 0],
            [1, 0, 1],];

        let result = left | right;

        assert_eq!(result, expected);

    }

    #[test]
    fn test_part_one() {
        let input = "30373
25512
65332
33549
35390";

        let result = part_one(input);

        assert_eq!(result, 21);
    }
}

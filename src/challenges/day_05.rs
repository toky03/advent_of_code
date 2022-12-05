use regex::Regex;
use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::ops::DerefMut;
use std::rc::Rc;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
struct Ship {
    cargos: BTreeMap<String, Vec<String>>,
}

impl Ship {
    pub fn new(cargos: BTreeMap<String, Vec<String>>) -> Self {
        Ship {
            cargos
        }
    }


    pub fn apply_instruction(&self, move_instruction: MoveInstruction) -> Self {
        let mut cargos = self.cargos.clone();

        let mut from: Vec<String> = cargos.get(move_instruction.from_cargo.as_str()).unwrap().clone();
        let mut to: Vec<String> = cargos.get(move_instruction.to_cargo.as_str()).unwrap().clone();

        for _ in 0..move_instruction.moved_containers {
            let moved_container = from.pop().unwrap();
            to.push(moved_container);
        }
        cargos.insert(move_instruction.from_cargo, from);
        cargos.insert(move_instruction.to_cargo, to);
        Ship::new(cargos)
    }
}

struct ShipSetup {
    setup_lines: Vec<String>,
}

impl ShipSetup {
    fn new() -> Self {
        ShipSetup {
            setup_lines: Vec::new(),
        }
    }

    fn add_setup_line(mut self: Self, input: &str) {
        self.setup_lines.push(input.to_string());
    }
}

fn string_to_cargo_names(input: &str) -> Vec<String> {
    input.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_string()).collect()
}

pub fn apply_all_instructions(input_lines: Vec<&str>) -> String {
    let initial_cargos = BTreeMap::from([(String::from("1"), string_to_vec_of_strings("DLVTMHF")),
        ("2".to_string(), string_to_vec_of_strings("HQGJCTNP")),
        ("3".to_string(), string_to_vec_of_strings("RSDMPH")),
        ("4".to_string(), string_to_vec_of_strings("LBVF")),
        ("5".to_string(), string_to_vec_of_strings("NHGLQ")),
        ("6".to_string(), string_to_vec_of_strings("WBDGRMP")),
        ("7".to_string(), string_to_vec_of_strings("GMNRCHLQ")),
        ("8".to_string(), string_to_vec_of_strings("CLW")),
        ("9".to_string(), string_to_vec_of_strings("RDLQJZMT"))]);
    apply_instructions_on_initial_ship(initial_cargos, input_lines)

}

pub fn apply_instructions_on_initial_ship(initial_cargos: BTreeMap<String, Vec<String>>, input_lines: Vec<&str>) -> String {
    let initial_ship = Ship::new(initial_cargos);
    let output: Ship = input_lines.into_iter().map(|instruction| MoveInstruction::from_str(instruction).unwrap()).fold(initial_ship, |acc, instruction| {
        acc.apply_instruction(instruction)
    });
    println!("output: {:?}", output);
    output.cargos.values().into_iter().map(|cargo| cargo.get(cargo.len() -1 ).unwrap().to_string()).collect()
}

#[derive(PartialEq, Debug)]
struct MoveInstruction {
    moved_containers: i32,
    from_cargo: String,
    to_cargo: String,
}

impl FromStr for MoveInstruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_regex: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = instruction_regex.captures(s).unwrap();
        let number_moved = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let from_cargo = caps.get(2).unwrap().as_str().to_string();
        let to_cargo = caps.get(3).unwrap().as_str().to_string();
        Ok(Self {
            moved_containers: number_moved,
            from_cargo,
            to_cargo,
        })
    }
}

fn string_to_vec_of_strings(input: &str) -> Vec<String> {
    input.chars().into_iter().map(|char| {char.to_string()}).collect()
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::challenges::day_05::{MoveInstruction, Ship};
    use super::string_to_cargo_names;
    use super::apply_instructions_on_initial_ship;
    use super::string_to_vec_of_strings;
    use std::str::FromStr;

    #[test]
    fn read_instructions_from_string() {
        let instruction: MoveInstruction = MoveInstruction::from_str("move 1 from 3 to 5").unwrap();
        assert_eq!(
            instruction,
            MoveInstruction {
                moved_containers: 1,
                to_cargo: String::from("5"),
                from_cargo: String::from("3"),
            }
        );
    }

    #[test]
    fn test_application() {
        let initial_cargos = BTreeMap::from([(String::from("1"), string_to_vec_of_strings("NZ")),
            ("2".to_string(), string_to_vec_of_strings("MCD")),
            ("3".to_string(), string_to_vec_of_strings("P"))]);
        let expected_cargos = BTreeMap::from([(String::from("1"), string_to_vec_of_strings("C")),
            ("2".to_string(), string_to_vec_of_strings("M")),
            ("3".to_string(), string_to_vec_of_strings("PDNZ"))]);

        let input_lines = vec!["move 1 from 2 to 1", "move 3 from 1 to 3", "move 2 from 2 to 1", "move 1 from 1 to 2"];



        assert_eq!(apply_instructions_on_initial_ship(initial_cargos, input_lines),"CMZ");

    }

    #[test]
    fn test_cargo_names() {
        assert_eq!(string_to_cargo_names(" 1 2 3 4   5 6 7"), vec!["1", "2", "3", "4", "5", "6", "7"])
    }

    #[test]
    fn test_ship_setup() {
        let cargos = BTreeMap::from([("1".to_string(), vec!["Z".to_string(), "N".to_string()]),
            ("2".to_string(), vec!["M".to_string(), "C".to_string(), String::from("D")]),
            ("3".to_string(), vec![String::from("P")])
        ]);

        let ship = Ship::new(cargos);
        let ship_new = ship.apply_instruction(MoveInstruction::from_str("move 1 from 2 to 1").unwrap());

        let expected_cargos = BTreeMap::from([("1".to_string(), vec!["Z".to_string(), "N".to_string(), String::from("D")]),
            ("2".to_string(), vec!["M".to_string(), "C".to_string()]),
            ("3".to_string(), vec![String::from("P")])
        ]);

        assert_eq!(ship_new, Ship::new(expected_cargos));
    }
}

use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::iter::Map;

struct Terminal {
    accumulated_lines: String,
    current_directory: String,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            accumulated_lines: String::from("root/"),
            current_directory: String::from("root/"),
        }
    }

    fn change_up(&self) -> Terminal {
        let mut directories: Vec<&str> = self
            .current_directory
            .split("/").collect();
        directories.pop();
        let super_directory: String = directories.into_iter().map(|dir| dir.to_string()).collect::<Vec<String>>().join("/");
        Terminal {
            accumulated_lines: format!(
                "{}\n{}",
                self.accumulated_lines,
                self.current_directory.clone()
            ),
            current_directory: format!("{super_directory}/"),
        }
    }

    fn add_file(&self, file_size: &str) -> Terminal {
        let prefix = if self.accumulated_lines.ends_with("/") {
            "|"
        } else {
            ","
        };
        let accumulated_lines = format!("{}{prefix}{file_size}", self.accumulated_lines);
        Terminal {
            accumulated_lines,
            current_directory: self.current_directory.clone(),
        }
    }

    fn change_directory(&self, directory_name: &str) -> Terminal {
        let current_directory = format!("{}{directory_name}/", self.current_directory.clone());
        Terminal {
            accumulated_lines: format!("{}\n{}", self.accumulated_lines, current_directory.clone()),
            current_directory,
        }
    }
}

fn parse_input(input_lines: Vec<&str>) -> String {
    let file_regex = Regex::new(r"(\d+) (\w+(\.\w+)*)").unwrap();
    let change_directory_regex = Regex::new(r"\$\s*cd\s*(\w+)").unwrap();
    let initial_terminal = Terminal::new();
    input_lines
        .into_iter()
        .fold(initial_terminal, |curr, command| {
            if file_regex.is_match(command) {
                let caps = file_regex.captures(command).unwrap();
                let file_size = caps.get(1).unwrap().as_str();
                return curr.add_file(file_size);
            }
            if command == "$ cd .." {
                return curr.change_up();
            }
            if change_directory_regex.is_match(command) {
                let caps = change_directory_regex.captures(command).unwrap();
                let directory = caps.get(1).unwrap().as_str();
                return curr.change_directory(directory);
            }
            curr
        })
        .accumulated_lines
}

fn count_size(input_lines: Vec<&str>) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for line in input_lines.into_iter() {
        let mut directories = line.split("|");
        let directory_names = directories.next().unwrap();
        if let Some(sizes) = directories.next() {
            for directory_name in extract_directory_names(directory_names) {
                let size = map.entry(directory_name).or_insert(0);
                *size += sum_of_strings(sizes);
            }
        }
    }
    map
}

fn sum_of_strings(sizes: &str) -> u32 {
    sizes
        .split(",")
        .filter(|split| !split.is_empty())
        .map(|size| size.parse::<u32>().unwrap())
        .sum()
}

fn extract_directory_names(input: &str) -> Vec<String> {
    input
        .split("/")
        .map(|directory| directory.to_string())
        .filter(|split| !split.is_empty())
        .collect()
}

fn filter_directories_with_maximum(
    map: HashMap<String, u32>,
    maximum: u32,
) -> HashMap<String, u32> {
    map.into_iter()
        .filter(|(key, value)| value.clone() <= maximum)
        .collect()
}

pub fn assembly(input_lines: Vec<&str>) -> u32 {
    let input = parse_input(input_lines);
    let map = count_size(input.lines().collect());
    println!("map {:?}", map);
    filter_directories_with_maximum(map, 100000)
        .into_iter()
        .map(|(k, v)| v)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::assembly;
    use super::count_size;
    use super::parse_input;
    use super::sum_of_strings;
    use std::collections::HashMap;

    #[test]
    fn test_assembly() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(assembly(input.lines().collect()), 95437);
    }

    #[test]
    fn test_sum_of_strings() {
        assert_eq!(sum_of_strings(""), 0);
        assert_eq!(sum_of_strings("1,"), 1);
        assert_eq!(sum_of_strings("2,3"), 5);
    }

    #[test]
    fn test_parse_input() {
        let input = "$ cd /
$ ls
dir abab
14848514 b.txt
8504156 c.dat
dir d
$ cd abab
$ ls
dir ef
29116 f
2557 g
62596 h.lst
$ cd ef
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let splitted: Vec<&str> = input.lines().collect();

        let result = parse_input(splitted);

        let expected = "root/|14848514,8504156
root/abab/|29116,2557,62596
root/abab/ef/|584
root/abab/ef/
root/abab/
root/d/|4060174,8033020,5626152,7214296";

        println!("{result}");

        assert_eq!(result, expected)
    }

    #[test]
    fn test_map_creation() {
        let input: Vec<&str> = "root/|14848514,8504156,
root/a/|29116,2557,62596,
root/a/e/|584,
root/a/e/
root/a/
root/d/|4060174,8033020,5626152,7214296,"
            .lines()
            .collect();

        let expected: HashMap<String, u32> = HashMap::from([
            ("a".to_string(), 94853),
            ("e".to_string(), 584),
            ("d".to_string(), 24933642),
            ("root".to_string(), 48381165),
        ]);

        assert_eq!(count_size(input), expected);
    }
}

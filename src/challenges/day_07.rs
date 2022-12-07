use regex::Regex;
use std::collections::BTreeMap;

struct Terminal {
    accumulated_lines: String,
    current_directory: String,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            accumulated_lines: String::from("/"),
            current_directory: String::from("/"),
        }
    }

    fn change_up(&self) -> Terminal {
        let current_directory = &self.current_directory.as_str()[0..(self.current_directory.len() - 2)];
        Terminal {
            accumulated_lines: format!("{}\n{}", self.accumulated_lines, self.current_directory.clone()),
            current_directory: current_directory.to_string(),
        }
    }

    fn add_file(&self, file_size: &str) -> Terminal {
        let accumulated_lines = format!("{}{file_size},", self.accumulated_lines);
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
    let file_regex = Regex::new(r"(\d+) (\w+\.\w+)").unwrap();
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

#[cfg(test)]
mod tests {
    use super::parse_input;

    #[test]
    fn test_parse_input() {
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

        let splitted: Vec<&str> = input.lines().collect();

        let result = parse_input(splitted);

        let expected = "/14848514,8504156,
/a/29116,2557,62596,
/a/e/584,
/
/d/4060174,8033020,5626152 d,7214296,";

        println!("{result}");

        assert_eq!(result, expected)
    }
}

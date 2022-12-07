use std::collections::BTreeMap;
use regex::Regex;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u128,
}

impl File {
    fn new(name: String, size: u128) -> Self {
        File {
            name,
            size,
        }
    }
}

#[derive(Debug, Clone)]
struct Directory<'a> {
    directory_name: String,
    parent_directory: Option<Box<&'a Directory<'a>>>,
    files: Vec<File>,
    sub_directories: Vec<Directory<'a>>,
}

impl Directory<'_> {

    fn new<'a>(directory_name: String, parent_directory: Option<Box<&'a Directory<'a>>>) -> Directory<'a> {
        Directory {
            directory_name,
            parent_directory,
            files: Vec::new(),
            sub_directories: Vec::new(),
        }
    }

    fn add_file(&self, file_name: String, size: u128) -> Self {
        let mut files = self.files.clone();
        files.push(File::new(file_name, size));
        Directory {
            files,
            sub_directories: self.sub_directories.to_vec(),
            parent_directory: self.parent_directory.clone(),
            directory_name: self.directory_name.clone(),
        }
    }

    fn add_directory(&self, directory_name: String) -> Self {
        let mut directories = self.sub_directories.clone();
        directories.push(Directory::new(directory_name, Some(Box::new(self))));
        Directory {
            files: self.files.clone(),
            sub_directories: directories,
            parent_directory: self.parent_directory.clone(),
            directory_name: self.directory_name.clone(),
        }
    }

}

fn parse_input(input_lines: Vec<&str>) -> Directory {
    let file_regex = Regex::new(r"(\d+) (\w+\.\w+)").unwrap();
    let root_directory = Directory::new(String::from("/"), None);
    input_lines.into_iter().fold(root_directory, |curr, command| {
        if command.starts_with("dir ") {
            return curr.add_directory(command.replace("dir ", "").to_string());
        }
        if file_regex.is_match(command) {
            let caps = file_regex.captures(command).unwrap();
            return curr.add_file(caps.get(2).unwrap().as_str().to_string(), caps.get(1).unwrap().as_str().parse::<u128>().unwrap())
        }
        curr
    })
}


#[cfg(test)]
mod tests {
    use super::Directory;
    use super::parse_input;

    #[test]
    fn test_parse_input() {
        let input =
            "$ cd /
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

        let result: Directory = parse_input(splitted);
        let expected: Directory = Directory::new(String::from("/"), None);

        println!("{:?}", result);
        
        assert_eq!(result.directory_name, expected.directory_name)
    }
}

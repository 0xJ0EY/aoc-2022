use std::{rc::{Weak, Rc}, cell::RefCell, str::FromStr};

#[derive(Debug)]
struct File {
    pub size: usize,
}

#[derive(Debug)]
struct Directory {
    pub parent: Option<Weak<RefCell<Directory>>>,
    pub name: String,
    pub files: Vec<File>,
    pub subdirectories: Vec<Rc<RefCell<Directory>>>
}

impl Directory {
    fn total_size(&self) -> usize {
        let mut sum = 0;

        sum += self.files.iter().map(|x| x.size).sum::<usize>();
        sum += self.subdirectories.iter().map(|x| x.borrow().total_size()).sum::<usize>();
        
        sum
    }
}

#[derive(Debug)]
struct FileSystem {
    pub root: Rc<RefCell<Directory>>,
    pub current_node: Weak<RefCell<Directory>>
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Rc::new(RefCell::new(Directory {
            parent: None,
            name: String::from("\\"),
            files: vec![],
            subdirectories: vec![]
        }));

        let current_node = Rc::downgrade(&root);

        FileSystem { root, current_node }
    }

    fn go_to_root(&mut self) {
        self.current_node = Rc::downgrade(&self.root);
    }

    fn go_up_directory(&mut self) {
        let node = self.current_node.upgrade().unwrap();
        let directory = node.borrow();

        let parent = directory.parent.as_ref().expect("Current node has no parent node");

        self.current_node = Rc::downgrade(&parent.upgrade().unwrap());
    }

    fn go_to_directory(&mut self, directory_name: &str) {
        let node = self.current_node.upgrade().unwrap();
        let directory = node.borrow();

        let destination = directory.subdirectories.iter()
            .find(|&dir| dir.borrow().name == directory_name)
            .expect("No node found with the given name");

        self.current_node = Rc::downgrade(destination);
    }

    fn create_directory(&mut self, directory_name: &str) {
        let node = self.current_node.upgrade().unwrap();
        
        node.borrow_mut().subdirectories.push(Rc::new(RefCell::new(Directory {
            parent: Some(Rc::downgrade(&node)),
            name: String::from(directory_name),
            files: vec![],
            subdirectories: vec![],
        })));
    }

    fn create_file(&mut self, size: usize) {
        let node = self.current_node.upgrade().unwrap();
        
        node.borrow_mut().files.push(File { size });
    }
}

impl FromStr for FileSystem {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut fs = FileSystem::new();

        for line in input.split_terminator('\n') {
            let parts = line.split(' ').collect::<Vec<_>>();
    
            let command_parts = (
                *parts.get(0).unwrap_or(&""),
                *parts.get(1).unwrap_or(&""),
                *parts.get(2).unwrap_or(&""),
            );
    
            match command_parts {
                ("$", "cd", "/")    => fs.go_to_root(),
                ("$", "cd", "..")   => fs.go_up_directory(),
                ("$", "cd", _)      => fs.go_to_directory(command_parts.2),
                ("$", "ls", _)      => continue,
                ("dir", _, _)       => fs.create_directory(command_parts.1),
                (_, _, _)           => fs.create_file(command_parts.0.parse().unwrap())
            }
        }
    
        Ok(fs)
    }
}

fn find_clearable_space(directory: &Directory) -> usize {
    let mut sum = 0;
    let current_directory_size = directory.total_size();

    sum += directory.subdirectories.iter()
        .map(|x| find_clearable_space(&x.as_ref().borrow()))
        .sum::<usize>();

    if current_directory_size <= 100_000 {
        sum += current_directory_size;
    }

    sum
}

const AVAILABLE_DISK_SPACE: usize   = 70000000;
const REQUIRED_UNUSED_SPACE: usize  = 30000000;

fn find_update_space(directory: &Directory, required: &usize) -> Option<usize> {
    let current_directory = directory.total_size();
    if current_directory < *required { return None }

    let mut lowest = directory.subdirectories.iter()
        .map(|x| find_update_space(&x.as_ref().borrow(), required))
        .filter(|x| x.is_some())
        .collect::<Vec<Option<usize>>>();

    if !lowest.is_empty() {
        lowest.sort();
        return *lowest.first().unwrap();
    } else {
        return Some(current_directory)    
    }
}

fn part1(input: &str) -> usize {
    let fs = FileSystem::from_str(input).unwrap();
    let result = find_clearable_space(&fs.root.as_ref().borrow());
    result
}


fn part2(input: &str) -> usize {
    let fs      = FileSystem::from_str(input).unwrap();
    let used_space  = &fs.root.as_ref().borrow().total_size();
    let unused_space    = AVAILABLE_DISK_SPACE - used_space;
    let space_to_find   = REQUIRED_UNUSED_SPACE - unused_space;

    let closest = find_update_space(&fs.root.as_ref().borrow(), &space_to_find);
    closest.unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

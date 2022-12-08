use std::{collections::HashMap};

fn main() {
    let input = include_str!("day7_input.txt");

    let mut lines = input.lines().map(|x|x.split_ascii_whitespace());

    let mut path_components = Vec::<String>::new();

    let mut root = Directory::new();
    
    while let Some(mut line) = lines.next() {
        match line.next().unwrap() {
            "$" => {
                match line.next().unwrap() {
                    "cd" => {
                        match line.next().unwrap() {
                            "/" => path_components.clear(),
                            ".." => { path_components.pop(); },
                            dir => path_components.push(dir.to_string())
                        };
                    }
                    "ls" => (),
                    _ => panic!()
                };
            },
            other => {
                if let Ok(number) = str::parse::<usize>(other) {
                    path_components.iter()
                            .fold(&mut root, |a, name| {
                                if a.child_dirs.contains_key(name) {
                                    a.child_dirs.get_mut(name).unwrap()
                                } else {
                                    a.child_dirs.insert(name.to_string(), Directory::new());
                                    a.child_dirs.get_mut(name).unwrap()
                                }
                            }).insert_file(line.next().unwrap().to_string(), number);
                }
            }
        }
    }

    println!("Task 1: {}", root.task_1());
    println!("Task 2: {}", root.task_2());
}

#[derive(Debug)]
struct Directory {
    child_dirs: HashMap<String, Directory>,
    child_files: HashMap<String, usize>
}
impl Directory {
    fn new() -> Self {
        Directory { child_dirs: HashMap::new(), child_files: HashMap::new() }
    }
    fn insert_file(&mut self, name: String, size: usize) {
        self.child_files.insert(name, size);
    }
    fn pretty(&self, name: &str) -> String {
        fn pretty(me: &Directory, name: &str, depth: usize) -> String {
            let mut s = String::new();
            for _ in 0..depth {
                s += &format!(" ")
            }
            s += &format!("- {}\n", name.to_uppercase());
            for (child_name, child_dir) in &me.child_dirs {
                s += &pretty( child_dir, child_name, depth + 2)
            }
            for (child_name, child_size) in &me.child_files {
                for _ in 0..depth + 2 {
                    s += &format!(" ")
                }
                s += &format!("- {} (file={})\n", child_name, child_size);
            }
            s
        }
        pretty(self, name, 0)
    }
    fn my_size_indirect(&self) -> usize {
        self.child_dirs.iter().map(|(_,d)|d.my_size_indirect()).sum::<usize>() 
        + self.child_files.iter().map(|(_,f)|f).sum::<usize>()
    }
    fn task_1(&self) -> usize {
        let indirect_size = self.my_size_indirect();
        let mut i = if indirect_size <= 100_000 {
            indirect_size
        } else {
            0
        };
        for (_, d) in &self.child_dirs {
            i += d.task_1()
        }
        i
    }
    fn task_2(&self) -> usize {
        const TOTAL_DISK_SPACE: usize = 70_000_000;
        const REQ_DISK_SPACE: usize = 30_000_000;
        let used_space = self.my_size_indirect();
        let remaining_space = TOTAL_DISK_SPACE - used_space;
        let minimum_to_free = REQ_DISK_SPACE - remaining_space;

        fn task_2_impl(me: &Directory, mut current: usize, minimum_to_free: usize) -> usize {
            let my_size = me.my_size_indirect();
            if my_size > minimum_to_free && my_size < current {
                current = my_size
            }
            for (_, d) in &me.child_dirs {
                let result = task_2_impl(d, current, minimum_to_free);
                if result < current {
                    current = result
                }
            }
            current
        }
        
        task_2_impl(self, usize::MAX, minimum_to_free)
    }
}
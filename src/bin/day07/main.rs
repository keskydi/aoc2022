use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Ressource {
    File(usize),
    Directory(HashMap<String, Ressource>),
}

impl Ressource {
    fn new_directory(hash_map: HashMap<String, Ressource>) -> Self {
        Ressource::Directory(hash_map)
    }

    fn new_file(value: usize) -> Self {
        Ressource::File(value)
    }
}

fn level_1(input: &str) -> usize {
    let tree = explore_dir(&mut input.split("$ ").skip(2).map(|x| x.to_string()));

    tree.iter()
        .filter_map(|(_, res)| match res {
            Ressource::File(_) => None,
            Ressource::Directory(dir) => {
                let (_, acc) = calc_child(dir);
                Some(acc)
            }
        })
        .flatten()
        .filter(|&x| x < 100_000)
        .sum()
}

fn level_2(input: &str) -> usize {
    let tree = explore_dir(&mut input.split("$ ").skip(2).map(|x| x.to_string()));

    let target = 40_000_000;
    let mut current = 0;

    let mut values = tree
        .iter()
        .filter_map(|(_, res)| match res {
            Ressource::File(value) => {
                current += value;
                None
            }
            Ressource::Directory(dir) => {
                let (value, acc) = calc_child(dir);
                current += value;
                Some(acc)
            }
        })
        .flatten()
        .collect::<Vec<usize>>();
    values.sort();
    *values.iter().find(|x| **x > (current - target)).unwrap()
}

fn calc_child(tree: &HashMap<String, Ressource>) -> (usize, Vec<usize>) {
    let mut acc = vec![];

    let current = tree.iter().fold(0, |current, (_, res)| match res {
        Ressource::File(val) => current + val,
        Ressource::Directory(dir) => {
            let (size_child, ac) = calc_child(dir);
            acc.extend(ac.iter());
            current + size_child
        }
    });
    acc.push(current);
    (current, acc)
}

fn explore_dir<'a, I: Iterator<Item = String>>(cmds: &'a mut I) -> HashMap<String, Ressource> {
    let mut childs: HashMap<String, Ressource> = HashMap::new();

    while let Some(value) = cmds.next() {
        match value.as_str().trim() {
            cmd if cmd.starts_with("ls") => {
                // dir.g
                for line in value.lines().skip(1) {
                    let (tipe, name) = line.split_once(" ").unwrap();

                    if tipe == "dir" {
                        childs.insert(name.trim().into(), Ressource::new_directory(HashMap::new()));
                    } else {
                        childs.insert(
                            name.trim().into(),
                            Ressource::new_file(tipe.parse::<usize>().unwrap()),
                        );
                    }
                }
            }
            "cd .." => {
                break;
            }
            cmd if cmd.starts_with("cd") => {
                let dir_name = cmd.strip_prefix("cd ").unwrap();
                match childs.get_mut(dir_name) {
                    Some(Ressource::Directory(res)) => res.extend(explore_dir(cmds)),
                    Some(_) => panic!("Error this is supposed to be a file"),
                    None => {
                        childs.insert(
                            dir_name.trim().into(),
                            Ressource::new_directory(explore_dir(cmds)),
                        );
                    }
                };
            }
            cmd => {
                panic!("Command not parse :'{cmd}'")
            }
        }
    }
    childs
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day07/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}

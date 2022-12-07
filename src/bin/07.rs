use std::collections::HashMap;

use itertools::Itertools;
#[derive(Debug, Default, Clone)]
struct Folder {
    pub name: String,
    pub value: Option<i128>,
}

impl PartialEq<Folder> for Folder {
    fn eq(&self, other: &Folder) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
struct Node<Folder>
where
    Folder: PartialEq,
{
    idx: usize,
    val: Folder,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<Folder> Node<Folder>
where
    Folder: PartialEq,
{
    fn new(idx: usize, val: Folder) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl ArenaTree<Folder>
where
    Folder: PartialEq,
{
    fn node(&mut self, val: Folder) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn get_total_size(&self) -> i128 {
        self.arena
            .iter()
            .filter(|x| x.children.is_empty())
            .map(|x| x.val.value.unwrap())
            .sum()
    }

    fn compute_node_size(&mut self, threshold: i128) {
        let arena = self.arena.clone();

        let leafs = arena.iter().filter(|x| x.children.is_empty());

        let mut map: HashMap<usize, i128> = HashMap::new();
        for leaf in leafs {
            if let Some(x) = map.get_mut(&leaf.parent.unwrap()) {
                *x += leaf.val.value.unwrap();
            } else {
                map.insert(leaf.parent.unwrap(), leaf.val.value.unwrap());
            }
        }

        loop {
            let mut nested_map: HashMap<usize, i128> = HashMap::new();

            for (key, val) in map.iter() {
                let prev = self.arena[*key].val.value.unwrap_or(0);
                self.arena[*key].val.value = Some(*val + prev);
                let new_val = self.arena[*key].val.value.unwrap_or(0);
                // dbg!(self.arena[*key].clone());
                let parent = self.arena[*key].parent;
                if parent.is_some() {
                    if let Some(x) = nested_map.get_mut(&parent.unwrap()) {
                        *x += new_val;
                    } else {
                        nested_map.insert(parent.unwrap(), new_val);
                    }
                }
            }

            map = nested_map;
            if map.values().filter(|v| v <= &&threshold).count() == 0 {
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i128> {
    let lines = input.split('\n');

    let mut tree: ArenaTree<Folder> = ArenaTree::default();
    let mut current: Option<usize> = None;

    for line in lines {
        let command: Vec<&str> = line.split(' ').collect();
        match command[0] {
            "$" => {
                if command[1] == "cd" {
                    if command[2] == ".." && current.is_some() {
                        current = tree.arena[current.unwrap()].parent
                    } else {
                        let mut path = "";
                        if current.is_some() {
                            path = &tree.arena[current.unwrap()].val.name;
                        }
                        let new = tree.node(Folder {
                            name: path.to_string() + "/" + &command[2].replace("/", "~"),
                            value: None,
                        });
                        if current.is_some() {
                            tree.arena[current.unwrap()].children.push(new);
                        }
                        tree.arena[new].parent = current;
                        current = Some(new);
                    }
                }
            }
            "dir" => {
                // do nothing
            }
            _ => {
                let mut path = "";
                if current.is_some() {
                    path = &tree.arena[current.unwrap()].val.name;
                }
                // create child leaf node
                let new = tree.node(Folder {
                    name: path.to_string() + "/" + command[1],
                    value: command[0].parse::<i128>().ok(),
                });
                if current.is_some() {
                    tree.arena[current.unwrap()].children.push(new);
                }
                tree.arena[new].parent = current;
            }
        }
    }

    tree.compute_node_size(100000_i128);

    let tmp: Vec<Folder> = tree
        .arena
        .iter()
        .filter(|x| !x.children.is_empty() && x.val.value < Some(100000))
        .map(|x| x.val.clone())
        .collect();
    Some(tmp.iter().map(|x| x.value.unwrap_or(0)).sum())
}

pub fn part_two(input: &str) -> Option<i128> {
    let lines = input.split('\n');

    let mut tree: ArenaTree<Folder> = ArenaTree::default();
    let mut current: Option<usize> = None;

    for line in lines {
        let command: Vec<&str> = line.split(' ').collect();
        match command[0] {
            "$" => {
                if command[1] == "cd" {
                    if command[2] == ".." && current.is_some() {
                        current = tree.arena[current.unwrap()].parent
                    } else {
                        let mut path = "";
                        if current.is_some() {
                            path = &tree.arena[current.unwrap()].val.name;
                        }
                        let new = tree.node(Folder {
                            name: path.to_string() + "/" + &command[2].replace("/", "~"),
                            value: None,
                        });
                        if current.is_some() {
                            tree.arena[current.unwrap()].children.push(new);
                        }
                        tree.arena[new].parent = current;
                        current = Some(new);
                    }
                }
            }
            "dir" => {
                // do nothing
            }
            _ => {
                let mut path = "";
                if current.is_some() {
                    path = &tree.arena[current.unwrap()].val.name;
                }
                // create child leaf node
                let new = tree.node(Folder {
                    name: path.to_string() + "/" + command[1],
                    value: command[0].parse::<i128>().ok(),
                });
                if current.is_some() {
                    tree.arena[current.unwrap()].children.push(new);
                }
                tree.arena[new].parent = current;
            }
        }
    }


    let min_space_to_empty = 30000000 - (70000000 - tree.get_total_size());
    
    
    tree.compute_node_size(30000000);

    let tmp: Option<i128> = tree
        .arena
        .iter()
        .filter(|x| !x.children.is_empty() && x.val.value >= Some(min_space_to_empty))
        .map(|x| x.val.value.unwrap())
        .min();
    tmp
    // None
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

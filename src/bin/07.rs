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

    fn compute_node_size(&mut self) {
        // if node.value.is_some() {
        //     return node.value.unwrap();
        // }

        // let idx = self.node(node);
        // let sum = self.arena[idx]
        //     .children.clone()
        //     .iter()
        //     .map(|&n| self.compute_node_size(self.arena[n].val.clone())).sum();
        // self.arena[idx].val.value = Some(sum);

        // sum

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

        loop{
            let mut nested_map: HashMap<usize, i128> = HashMap::new();
    
            for (key, val) in map.iter().filter(|(k,v)| v<=&&100000_i128) {
                self.arena[*key].val.value = Some(*val);
                let parent = self.arena[*key].parent;
                if parent.is_some(){
                    if let Some(x) = nested_map.get_mut(&parent.unwrap()) {
                        *x += *val;
                    } else {
                        nested_map.insert(parent.unwrap(), *val);
                    }
                }
            }
    
            map = nested_map;
            if map.values().filter(|v| v<=&&100000_i128).count() == 0{
                break;
            }
        }


        
        dbg!(map.values().filter(|v| v<=&&100000_i128).count());
        dbg!(map.values().count());

        // for leaf in leafs {
        //     if leaf.parent.is_some(){
        //         self.arena[leaf.parent.unwrap()].val.value = Some(self.arena[leaf.parent.unwrap()].val.value.unwrap_or(0) + leaf.val.value.unwrap_or(0));
        //         let mut current = self.arena[leaf.parent.unwrap()].clone();
        //         loop {
        //             if current.parent.is_none(){
        //                 break;
        //             }
        //             self.arena[current.parent.unwrap()].val.value = Some(self.arena[current.parent.unwrap()].val.value.unwrap_or(0) + current.val.value.unwrap_or(0));
        //             current = self.arena[current.parent.unwrap()].clone();
        //         }
        //     }
        // }
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
                        let new = tree.node(Folder {
                            name: command[2].to_string(),
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
                // create child leaf node
                let new = tree.node(Folder {
                    name: command[1].to_string(),
                    value: command[0].parse::<i128>().ok(),
                });
                if current.is_some() {
                    tree.arena[current.unwrap()].children.push(new);
                }
                tree.arena[new].parent = current;
            }
        }
    }

    tree.compute_node_size();

    // let tmp: Vec<Folder> = tree.arena.iter().filter(|x| !x.children.is_empty() && x.val.value < Some(100000)).map(|x| x.val.clone()).collect();

    // Some(tmp.iter().map(|x| x.value.unwrap_or(0)).sum())
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}

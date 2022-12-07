#[derive(Debug, Default, Clone)]
struct Folder {
    pub name: String,
    pub value: Option<i32>,
}

impl PartialEq<Folder> for Folder {
    fn eq(&self, other: &Folder) -> bool {
        self.name == other.name
    }
}

#[derive(Debug,Clone)]
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

    fn compute_node_size(&mut self, node: Folder)  {
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

        for leaf in leafs {
            if leaf.parent.is_some(){
                self.arena[leaf.parent.unwrap()].val.value = Some(self.arena[leaf.parent.unwrap()].val.value.unwrap_or(0) + leaf.val.value.unwrap_or(0));
                let mut current = self.arena[leaf.parent.unwrap()].clone();
                
            }
        }

    }
}

pub fn part_one(input: &str) -> Option<i32> {
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
                    value: command[0].parse::<i32>().ok(),
                });
                if current.is_some() {
                    tree.arena[current.unwrap()].children.push(new);
                }
                tree.arena[new].parent = current;
            }
        }
    }

    let root = tree
        .arena
        .iter()
        .find(|n| n.parent.is_none())
        .unwrap()
        .val
        .clone();

    tree.compute_node_size(root);

    
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

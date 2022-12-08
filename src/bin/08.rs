use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {

    let trees:Vec<Vec<char>> = input.split('\n').map(|v| v.chars().collect_vec()).collect();
    
    let outside_trees = (trees.len() + trees[0].len()) as i32;

    

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}

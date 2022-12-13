use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let packets = input
        .split("\n\n")
        .map(|p| p.split('\n').collect_vec())
        .collect_vec();

    let correct_indices: Vec<usize> = vec![];

    for p in packets {
        let left = p[0]; //  [1,[2,[3,[4,[5,6,7]]]],8,9]
        let right = p[1]; // [1,[2,[3,[4,[5,6,0]]]],8,9]

        
    }

    // I think I'm gonna do this in javascript

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}

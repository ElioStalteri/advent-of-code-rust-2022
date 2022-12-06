fn check_code(code: &str) -> bool{
    for (i1, c1) in code.chars().enumerate() {
        for (i2, c2) in code.chars().enumerate() {
            if i1 != i2 && c1 == c2 {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<i32> {
    for (i,c) in input.chars().enumerate() {
        if i + 4 < input.len() && check_code(&input[i..(i+4)]){
            return Some((i + 4) as i32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<i32> {
    for (i,c) in input.chars().enumerate() {
        if i + 14 < input.len() && check_code(&input[i..(i+14)]){
            return Some((i + 14) as i32);
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}

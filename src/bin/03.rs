use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let rucksacks = input.split("\n");

    let mut priority: HashMap<char, i32> = HashMap::from([]);

    for (i, el) in (b'a'..=b'z').enumerate() {
        priority.insert(el as char, i32::try_from(i + 1).unwrap());
    }

    for (i, el) in (b'A'..=b'Z').enumerate() {
        priority.insert(el as char, i32::try_from(i + 1 + 26).unwrap());
    }

    // println!("{}", (b'a'..=b'z').fold(String::new(), |acc, c| format!("{} {}", acc, c as char)));

    let mut common_letters: Vec<char> = vec![];

    for rucksack in rucksacks {
        let sack_l = &rucksack[0..rucksack.len() / 2];
        let sack_r = &rucksack[rucksack.len() / 2..rucksack.len()];

        for letter in sack_l.chars() {
            if sack_r.find(letter).is_some() {
                common_letters.push(letter);
                break;
            }
        }
    }
    let res = common_letters.iter().fold(0, |acc,v| acc + priority.get(v).unwrap_or(&0));

    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {

    let rucksacks = input.split("\n");

    let mut priority: HashMap<char, i32> = HashMap::from([]);

    for (i, el) in (b'a'..=b'z').enumerate() {
        priority.insert(el as char, i32::try_from(i + 1).unwrap());
    }

    for (i, el) in (b'A'..=b'Z').enumerate() {
        priority.insert(el as char, i32::try_from(i + 1 + 26).unwrap());
    }

    // println!("{}", (b'a'..=b'z').fold(String::new(), |acc, c| format!("{} {}", acc, c as char)));

    let mut common_letters: Vec<char> = vec![];


    let elf_groups = rucksacks.enumerate().fold(vec![], |mut acc,(i,v)| {
        match i % 3 {
            0 => {
                acc.push(vec![v]);
            }
            _ => {
                acc.last_mut().unwrap().push(v);
            }
        }
        acc
    });


    for group in elf_groups {
        for letter in group[0].chars() {
            if group[1].find(letter).is_some() && group[2].find(letter).is_some() {
                common_letters.push(letter);
                break;
            }
        }
    }


    let res = common_letters.iter().fold(0, |acc,v| acc + priority.get(v).unwrap_or(&0));

    Some(res)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

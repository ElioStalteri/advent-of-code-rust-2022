use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<i128>,
    fomula_is_addition: bool,
    fomula_second_term: String,
    divisible_by: i128,
    if_true: usize,
    if_false: usize,
}

fn map_monkeys(input: &str) -> Vec<Monkey> {
    let re = Regex::new(
        r"\d+:\n\s{2}Starting items:(.*)\n\s{2}Operation:(.*)\n\s{2}Test:(.*)\n\s{4}If true:(.*)\n\s{4}If false:(.*)",
    )
    .unwrap();

    input
        .split("\nMonkey ")
        .map(|v| {
            let caps = re.captures(v).unwrap();

            let operation = caps.get(2).map_or("", |m| m.as_str());
            let additions = operation.contains('+');

            Monkey {
                items: caps
                    .get(1)
                    .map_or("", |m| m.as_str())
                    .split(':')
                    .last()
                    .unwrap()
                    .split(',')
                    .map(|v| v.trim().parse::<i128>().ok().unwrap())
                    .collect_vec(),
                fomula_is_addition: additions,
                fomula_second_term: operation.split(' ').last().unwrap().to_string(),
                divisible_by: caps
                    .get(3)
                    .map_or("", |m| m.as_str())
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<i128>()
                    .ok()
                    .unwrap(),
                if_true: caps
                    .get(4)
                    .map_or("", |m| m.as_str())
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .ok()
                    .unwrap(),
                if_false: caps
                    .get(5)
                    .map_or("", |m| m.as_str())
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .ok()
                    .unwrap(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i128> {
    let mut monkeys: Vec<Monkey> = map_monkeys(input);

    let mut monkeys_number_of_inpections :Vec<i128>= monkeys.iter().map(|_| 0).collect();


    // Monkey inspects an item with a worry level of 79.
    // Worry level is multiplied by 19 to 1501.
    // Monkey gets bored with item. Worry level is divided by 3 to 500.
    // Current worry level is not divisible by 23.
    // Item with worry level 500 is thrown to monkey 3.
    for _ in 1..21 {
        for im in 0..monkeys.len() {
            let mnk = monkeys[im].clone();
            for item in mnk.items.iter() {
                monkeys_number_of_inpections[im] += 1;
                let mut new_item:i128;
                if mnk.fomula_is_addition {
                    if mnk.fomula_second_term == "old" {
                        new_item = item + item;
                    }else{
                        new_item = item + mnk.fomula_second_term.parse::<i128>().ok().unwrap();
                    }
                } else if mnk.fomula_second_term == "old" {
                    new_item = item * item;
                }else{
                    new_item = item * mnk.fomula_second_term.parse::<i128>().ok().unwrap();
                }
                new_item /= 3;
                if new_item % mnk.divisible_by == 0 {
                    monkeys[mnk.if_true].items.push(new_item);
                }else{
                    monkeys[mnk.if_false].items.push(new_item);
                }
            }
            monkeys[im].items = vec![];
        }
    }


    monkeys_number_of_inpections.sort();
    monkeys_number_of_inpections.reverse();

    Some(monkeys_number_of_inpections[0] * monkeys_number_of_inpections[1])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut monkeys: Vec<Monkey> = map_monkeys(input);

    let mut monkeys_number_of_inpections :Vec<i128>= monkeys.iter().map(|_| 0).collect();

    
    // Monkey inspects an item with a worry level of 79.
    // Worry level is multiplied by 19 to 1501.
    // Monkey gets bored with item. Worry level is divided by 3 to 500.
    // Current worry level is not divisible by 23.
    // Item with worry level 500 is thrown to monkey 3.
    for _ in 1..1000 {
        for im in 0..monkeys.len() {
            let mnk = monkeys[im].clone();
            for item in mnk.items.iter() {
                monkeys_number_of_inpections[im] += 1;
                let new_item:i128;
                if mnk.fomula_is_addition {
                    if mnk.fomula_second_term == "old" {
                        new_item = item + item;
                    }else{
                        new_item = item + mnk.fomula_second_term.parse::<i128>().ok().unwrap();
                    }
                } else if mnk.fomula_second_term == "old" {
                    new_item = item * item;
                }else{
                    new_item = item * mnk.fomula_second_term.parse::<i128>().ok().unwrap();
                }
                dbg!(new_item);
                dbg!(mnk.divisible_by);
                dbg!(new_item % mnk.divisible_by);
                if new_item % mnk.divisible_by == 0 {
                    monkeys[mnk.if_true].items.push(new_item);
                }else{
                    monkeys[mnk.if_false].items.push(new_item);
                }
            }
            monkeys[im].items = vec![];
        }
    }


    // monkeys_number_of_inpections.sort();
    // monkeys_number_of_inpections.reverse();

    dbg!(monkeys_number_of_inpections);
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}

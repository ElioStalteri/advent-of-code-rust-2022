use std::num::ParseIntError;

use itertools::Itertools;

#[derive(Debug, Default, Clone)]
struct Command<'a> {
    op: &'a str,
    value: Option<i32>,
}

pub fn part_one(input: &str) -> Option<i32> {
    let commands: Vec<Command> = input
        .split('\n')
        .map(|v| {
            let vec_split = v.split(' ').collect_vec();
            let value = if vec_split.len() > 1 {
                vec_split[1].parse()
            } else {
                "0".parse()
            };
            Command {
                op: vec_split[0],
                value: if value.is_ok() { value.ok() } else { None },
            }
        })
        .collect();

    let mut buffer = 1;

    let cicle_array = [20, 60, 100, 140, 180, 220];
    let mut cicle_buffer_values: Vec<i32> = vec![];

    let mut current_command_pos = 0;
    let mut skip: bool = false;
    for cicle in 1..221 {
        if current_command_pos < commands.len() { 
            if cicle_array.contains(&cicle) {
                cicle_buffer_values.push(buffer);
            }
            let c = &commands[current_command_pos];
            match c.op {
                "noop" => {
                    current_command_pos += 1;
                }
                "addx" => {
                    if !skip {
                        skip = true;
                    }  else {
                        skip = false;
                        current_command_pos += 1;
                        buffer += c.value.unwrap();
                    }
                }
                _ => {}
            }
            
        }
    }

    Some(cicle_array.iter().enumerate().map(|(i,v)| v * cicle_buffer_values[i]).sum())
}

pub fn part_two(input: &str) -> Option<String> {
    let commands: Vec<Command> = input
        .split('\n')
        .map(|v| {
            let vec_split = v.split(' ').collect_vec();
            let value = if vec_split.len() > 1 {
                vec_split[1].parse()
            } else {
                "0".parse()
            };
            Command {
                op: vec_split[0],
                value: if value.is_ok() { value.ok() } else { None },
            }
        })
        .collect();

    let mut buffer = 1;

    let mut cicle_buffer_values: Vec<Vec<&str>> = vec![vec![]];

    let mut current_command_pos = 0;
    let mut skip: bool = false;
    for cicle in 1..241 {
        if current_command_pos < commands.len() { 
            let last_row = cicle_buffer_values.len() - 1;
            if cicle % 40 >= buffer && cicle % 40 <= buffer+2 {
                cicle_buffer_values[last_row].push("#");
            }else{
                cicle_buffer_values[last_row].push(".");
            }
            if cicle % 40 == 0{
                cicle_buffer_values.push(vec![]);
            }
            
            let c = &commands[current_command_pos];
            match c.op {
                "noop" => {
                    current_command_pos += 1;
                }
                "addx" => {
                    if !skip {
                        skip = true;
                    }  else {
                        skip = false;
                        current_command_pos += 1;
                        buffer += c.value.unwrap();
                    }
                }
                _ => {}
            }
            
        }
    }

    Some(cicle_buffer_values.iter().map(|b| b.join("")).collect::<Vec<String>>().join("\n"))
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######.....
".to_string()));
    }
}

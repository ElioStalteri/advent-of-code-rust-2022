use regex::Regex;

fn decode_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<&str>>) {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let tmp = input[1].replace('\n', "|");
    let tmp = re.replace_all(&tmp, "$1,$2,$3").to_string();
    let movements: Vec<Vec<i32>> = tmp
        .split('|')
        .map(|v| v.split(',').map(|p| p.parse::<i32>().unwrap()).collect())
        .collect();

    let re = Regex::new(r"([^|]{3})\s").unwrap();
    let tmp0 = input[0].replace('\n', "|");
    let tmp1 = re.replace_all(&tmp0, "$1,").to_string();
    let tmp2 = tmp1.replace("   ", "-").replace(['[', ']'], "");
    let mut raw_storage: Vec<Vec<&str>> = tmp2.split('|').map(|v| v.split(',').collect()).collect();
    raw_storage.pop();
    raw_storage.reverse();

    let mut storage: Vec<Vec<&str>> = raw_storage[0].iter().map(|_| vec![]).collect();

    for s in raw_storage {
        for (i, m) in s.iter().enumerate() {
            if m != &"-" {
                storage[i].push(*m);
            }
        }
    }

    (movements,storage)
}

pub fn part_one(input: &str) -> Option<String> {
    let (movements, mut storage) = decode_input(input);

    for movement in movements {
        let amount = movement[0];
        let from = movement[1] - 1;
        let to = movement[2] - 1;

        let mut buffer: Vec<&str> = vec![];

        for _ in 0..amount {
            if let Some(tmp) = storage[from as usize].pop() {
                buffer.push(tmp);
            }
        }

        for b in buffer {
            storage[to as usize].push(b)
        }
    }

    let res = storage
        .iter()
        .map(|v| *v.last().unwrap())
        .collect::<Vec<&str>>()
        .join("");

    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let tmp = input[1].replace('\n', "|");
    let tmp = re.replace_all(&tmp, "$1,$2,$3").to_string();
    let movements: Vec<Vec<i32>> = tmp
        .split('|')
        .map(|v| v.split(',').map(|p| p.parse::<i32>().unwrap()).collect())
        .collect();

    let re = Regex::new(r"([^|]{3})\s").unwrap();
    let tmp0 = input[0].replace('\n', "|");
    let tmp1 = re.replace_all(&tmp0, "$1,").to_string();
    let tmp2 = tmp1.replace("   ", "-").replace(['[', ']'], "");
    let mut raw_storage: Vec<Vec<&str>> = tmp2.split('|').map(|v| v.split(',').collect()).collect();
    raw_storage.pop();
    raw_storage.reverse();


    let mut storage: Vec<Vec<&str>> = raw_storage[0].iter().map(|_| vec![]).collect();

    for s in raw_storage {
        for (i, m) in s.iter().enumerate() {
            if m != &"-" {
                storage[i].push(*m);
            }
        }
    }


    for movement in movements {
        let amount = movement[0];
        let from = movement[1] - 1;
        let to = movement[2] - 1;

        let mut buffer: Vec<&str> = vec![];

        for _ in 0..amount {
            if let Some(tmp) = storage[from as usize].pop() {
                buffer.push(tmp);
            }
        }

        buffer.reverse();

        for b in buffer {
            storage[to as usize].push(b)
        }
    }

    let res = storage
        .iter()
        .map(|v| *v.last().unwrap())
        .collect::<Vec<&str>>()
        .join("");
        
    Some(res)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}

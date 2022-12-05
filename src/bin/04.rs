pub fn part_one(input: &str) -> Option<i32> {
    let pairs = input
        .split("\n")
        .map(|v| {
            v.split(',')
                .map(|zones_string| {
                    let zones = zones_string.split('-').map(|z| z.parse::<i32>().unwrap());

                    // (u8::from(zones[0])..u8::from(zones[1]))
                    zones.collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

    let mut included = 0;
    for pair in pairs {
        let range1 = &pair[0];
        let range2 = &pair[1];

        if range1[0] == range2[0] || range1[1] == range2[1] {
            included += 1;
        } else if range1[0] > range2[0] {
            if range1[1] < range2[1] {
                included += 1;
            }
        } else if range1[1] > range2[1] {
            included += 1;
        }
    }


    // dbg!((b'2'..=b'5').map(|v| v as char));

    // println!("{}", (b'2'..=b'5').fold(String::new(), |acc, c| format!("{} {}", acc, c as char)));

    // dbg!(pairs);

    Some(included)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = input
        .split("\n")
        .map(|v| {
            v.split(',')
                .map(|zones_string| {
                    let zones = zones_string.split('-').map(|z| z.parse::<i32>().unwrap());

                    // (u8::from(zones[0])..u8::from(zones[1]))
                    zones.collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

        
    let mut included = 0;
    for pair in pairs {
        let range1 = &pair[0];
        let range2 = &pair[1];

        if range1[0] <= range2[1] && range1[1] >= range2[0] {
            included += 1;
        }
    }


    Some(included)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

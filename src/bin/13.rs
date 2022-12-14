use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// #[derive(Serialize, Deserialize, Debug)]
// enum Value {
//     i32,
//     Array(Vec<Value>)
// }

fn check_difference(left: &Option<&Value>, right: &Option<&Value>) -> bool {
    /*
       for (const [leftItem, rightItem] of zip(left, right)) {
      if (leftItem === undefined && rightItem === undefined) return ComparisonResult.Undetermined;
      if (leftItem === undefined) return ComparisonResult.Correct;
      if (rightItem === undefined) return ComparisonResult.Incorrect;

      if (isNumber(leftItem) && isNumber(rightItem)) {
        if (leftItem === rightItem) continue;
        return leftItem < rightItem ? ComparisonResult.Correct : ComparisonResult.Incorrect;
      }

      const result = comparePackets(
        Array.isArray(leftItem) ? leftItem : [leftItem],
        Array.isArray(rightItem) ? rightItem : [rightItem],
      );

      if (result !== ComparisonResult.Undetermined) {
        return result;
      }
    }

    return ComparisonResult.Undetermined;
      */

    // dbg!(left, right);

    match (left, right) {
        (Some(Value::Array(_)), Some(Value::Array(_))) => {
            let left_arr = left.unwrap().as_array().unwrap();
            let right_arr = right.unwrap().as_array().unwrap();
            let lenght_arr = vec![left_arr.len(), right_arr.len()];
            let max_size = *lenght_arr.iter().max().unwrap();
            for i in 0..max_size {
                if !check_difference(
                    &left.unwrap().as_array().unwrap().get(i),
                    &right.unwrap().as_array().unwrap().get(i),
                ) {
                    return false;
                }
            }
        }
        (Some(Value::Array(_)), Some(Value::Number(_))) => {
            if left.unwrap().as_array().unwrap().len() == 0 {
                return false;
            }
            let new_l = left.unwrap().as_array().unwrap()[0].as_i64();
            let new_r = right.unwrap().as_i64();
            if new_l > new_r {
                return false;
            }
        }
        (Some(Value::Number(_)), Some(Value::Array(_))) => {
            if right.unwrap().as_array().unwrap().len() == 0 {
                return false;
            }
            let new_l = left.unwrap().as_i64();
            let new_r = right.unwrap().as_array().unwrap()[0].as_i64();
            if new_l > new_r {
                return false;
            }
        }
        (Some(Value::Number(_)), Some(Value::Number(_))) => {
            if left.unwrap().as_i64() > right.unwrap().as_i64() {
                return false;
            }
        }
        (None, None) => {}
        (None, _) => {}
        (_, None) => {
            return false;
        }
        _ => {}
    }

    return true;
}

pub fn part_one(input: &str) -> Option<usize> {
    let packets: Vec<Vec<Value>> = input
        .split("\n\n")
        .map(|p| {
            p.split('\n')
                .map(|v| serde_json::from_str(v).unwrap())
                .collect_vec()
        })
        .collect_vec();

    // DEBUG
    // let p = packets[1].clone();
    // let left = &p[0]; //  [1,[2,[3,[4,[5,6,7]]]],8,9]
    // let right = &p[1]; // [1,[2,[3,[4,[5,6,0]]]],8,9]
    // dbg!(check_difference(left, right, true));

    let mut correct_order = 0;

    for (i, p) in packets.iter().enumerate() {
        let left = &p[0];
        let right = &p[1];
        if check_difference(&Some(left), &Some(right)) {
            correct_order += i + 1;
        }
        dbg!(check_difference(&Some(left), &Some(right)));
    }

    dbg!(packets.len());
    // 476 too low
    Some(correct_order)
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
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Rock,
    Sand,
    Empty,
}

fn print_map(map: &Vec<Vec<Type>>) {
    let map_str = map
        .iter()
        .map(|x| {
            x.iter()
                .map(|p| match p {
                    Type::Rock => '#',
                    Type::Sand => 'o',
                    Type::Empty => '.',
                    _ => 'w',
                })
                .join("")
        })
        .join("\n");

    println!("{}", map_str);
}

fn get_map(input: &str, floor_y_delta: i32) -> (Vec<Vec<Type>>, (i32, i32)) {
    let min_y = 0;
    let mut max_y = 0;
    let mut min_x = 999999999;
    let mut max_x = 0;

    let mut rocks = input
        .split('\n')
        .map(|r| {
            r.split(" -> ")
                .map(|p| {
                    let points = p
                        .split(',')
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect_vec();
                    if points[1] > max_y {
                        max_y = points[1];
                    }
                    if points[0] > max_x {
                        max_x = points[0];
                    }
                    if points[0] < min_x {
                        min_x = points[0];
                    }
                    (points[0].clone(), points[1].clone())
                })
                .collect_vec()
        })
        .collect_vec();

    rocks = rocks
        .iter()
        .map(|p| p.iter().map(|(x, y)| (x - min_x, *y)).collect_vec())
        .collect_vec();

    max_y += 1;
    max_x += 1;

    max_y += floor_y_delta;

    let mut map =
        vec![vec![Type::Empty; max_y.try_into().unwrap()]; (max_x - min_x).try_into().unwrap()];

    for rock in &rocks {
        let mut prev = &rock[0];
        map[(prev.0) as usize][prev.1 as usize] = Type::Rock;
        for r in &rock[1..] {
            if r.0 == prev.0 {
                let max = if prev.1 > r.1 { prev.1 } else { r.1 };
                let min = if prev.1 < r.1 { prev.1 } else { r.1 };

                for i in min..(max + 1) {
                    map[(prev.0) as usize][i as usize] = Type::Rock;
                }
            } else {
                let max = if prev.0 > r.0 { prev.0 } else { r.0 };
                let min = if prev.0 < r.0 { prev.0 } else { r.0 };
                for i in min..(max + 1) {
                    map[i as usize][prev.1 as usize] = Type::Rock;
                }
            }
            prev = &r;
        }
    }

    let mut tmp_map =
        vec![vec![Type::Empty; (max_x - min_x).try_into().unwrap()]; max_y.try_into().unwrap()];
    for (ir, row) in map.iter().enumerate() {
        for (ic, v) in row.iter().enumerate() {
            tmp_map[ic][ir] = v.clone();
        }
    }

    (tmp_map.clone(), (500 - min_x, 0))
}

#[derive(Debug, Clone, PartialEq)]
enum FallType {
    Left,
    Right,
    Bottom,
    Stop,
}

fn compute_sand_fall(map: &mut Vec<Vec<Type>>, sand_pos: &(i32, i32)) -> FallType {
    let mut s_pos = sand_pos.clone();
    let bottom = map.len() - 1;
    for i in (s_pos.1 as usize)..bottom {
        if map[i + 1][s_pos.0 as usize] != Type::Empty {
            if s_pos.0 - 1 < 0 {
                return FallType::Left;
            }
            if map[i + 1][(s_pos.0 - 1) as usize] != Type::Empty {
                if s_pos.0 + 1 > (map[i + 1].len() - 1) as i32 {
                    return FallType::Right;
                }
                if map[i + 1][(s_pos.0 + 1) as usize] != Type::Empty {
                    map[i][s_pos.0 as usize] = Type::Sand;
                    return FallType::Stop;
                } else {
                    // go right
                    s_pos.0 += 1;
                }
            } else {
                // go left
                s_pos.0 -= 1;
            }
        }
    }
    return FallType::Bottom;
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut map, initial_sand_pos) = get_map(input, 0);
    // let term = console::Term::stdout();
    // term.hide_cursor().unwrap();
    // term.clear_screen().unwrap();
    while compute_sand_fall(&mut map, &initial_sand_pos) == FallType::Stop {
        // term.move_cursor_to(0, 0).unwrap();
        // print_map(&map);
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }
    // print_map(&map);

    Some(
        map.iter()
            .flatten()
            .filter(|v| **v == Type::Sand)
            .count()
            .try_into()
            .unwrap(),
    )
}

fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}

pub fn part_two(input: &str) -> Option<i32> {
    let (mut map, mut initial_sand_pos) = get_map(input, 2);
    let last_row_index = map.len() - 1 as usize;
    for r in map[last_row_index].iter_mut() {
        *r = Type::Rock;
    }
    // let term = console::Term::stdout();
    // term.hide_cursor().unwrap();
    // term.clear_screen().unwrap();
    while map[initial_sand_pos.1 as usize][initial_sand_pos.0 as usize] != Type::Sand {
        match compute_sand_fall(&mut map, &initial_sand_pos) {
            FallType::Left => {
                initial_sand_pos.0 +=1;
                map = map
                    .iter()
                    .enumerate()
                    .map(|(i, row)| {
                        let type_to_push = if i == last_row_index {
                            Type::Rock
                        } else {
                            Type::Empty
                        };
                        prepend(row.clone(), &[type_to_push])
                    })
                    .collect();
            }
            FallType::Right => {
                map = map
                    .iter()
                    .enumerate()
                    .map(|(i, row)| {
                        let type_to_push = if i == last_row_index {
                            Type::Rock
                        } else {
                            Type::Empty
                        };
                        let mut tmp = row.clone();
                        tmp.push(type_to_push);
                        tmp
                    })
                    .collect();
            }
            _ => {}
        }
        // term.move_cursor_to(0, 0).unwrap();
        // print_map(&map);
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }
    // print_map(&map);

    Some(
        map.iter()
            .flatten()
            .filter(|v| **v == Type::Sand)
            .count()
            .try_into()
            .unwrap(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}

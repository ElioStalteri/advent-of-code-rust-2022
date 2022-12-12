use itertools::Itertools;
use pathfinding::directed::dijkstra;

fn find_possible_routes(
    cur: (usize, usize),
    map: &Vec<Vec<i32>>,
    visited: &[(usize, usize)],
    check: fn(d: i32) -> bool,
) -> Vec<(usize, usize)> {
    let (y, x) = cur;
    let mut checks: Vec<(i32, i32)> = vec![];
    if y != 0 {
        checks.push((-1, 0));
    }
    if x != 0 {
        checks.push((0, -1));
    }
    if y != map.len() - 1 {
        checks.push((1, 0));
    }
    if x != map[0].len() - 1 {
        checks.push((0, 1));
    }
    let final_checks = checks
        .iter()
        .filter(|(dy, dx)| {
            !visited.iter().any(|(vy, vx)| {
                vy == &((y as i32 + dy) as usize) && vx == &((x as i32 + dx) as usize)
            })
        })
        .collect_vec();
    let mut res: Vec<(usize, usize)> = vec![];
    for (dy, dx) in final_checks {
        let delta_step = map[(y as i32 + dy) as usize][(x as i32 + dx) as usize] - map[y][x];
        if check(delta_step) {
            res.push(((y as i32 + dy) as usize, (x as i32 + dx) as usize))
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut initial: (usize, usize) = (0, 0);
    let mut finish: (usize, usize) = (0, 0);
    let map: Vec<Vec<i32>> = input
        .split('\n')
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        initial = (y, x);
                        visited.push((y, x));
                        (b'a' - 1) as i32 - (b'a' - 1) as i32
                    } else if c == 'E' {
                        finish = (y, x);
                        (b'z' + 1) as i32 - (b'a' - 1) as i32
                    } else {
                        (c as u8) as i32 - (b'a' - 1) as i32
                    }
                })
                .collect()
        })
        .collect();

    let start = initial;
    let end = finish;

    let check = |d: i32| d < 2;
    let check_complete = |&p: &(usize, usize)| p == end;

    let result = dijkstra::dijkstra(
        &start,
        |cur| {
            find_possible_routes(*cur, &map, &vec![], check)
                .into_iter()
                .map(|p| (p, 1))
        },
        check_complete,
    );

    return Some(result.unwrap().1 - 2);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut initial: (usize, usize) = (0, 0);
    let mut finish: (usize, usize) = (0, 0);
    let map: Vec<Vec<i32>> = input
        .split('\n')
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        initial = (y, x);
                        visited.push((y, x));
                        (b'a' - 1) as i32 - (b'a' - 1) as i32
                    } else if c == 'E' {
                        finish = (y, x);
                        (b'z' + 1) as i32 - (b'a' - 1) as i32
                    } else {
                        (c as u8) as i32 - (b'a' - 1) as i32
                    }
                })
                .collect()
        })
        .collect();

    let start = finish;

    let check = |d: i32| d > -2;
    let check_complete = |&p: &(usize, usize)| p.1 == 0;

    let result = dijkstra::dijkstra(
        &start,
        |cur| {
            find_possible_routes(*cur, &map, &vec![], check)
                .into_iter()
                .map(|p| (p, 1))
        },
        check_complete,
    );

    return Some(result.unwrap().1 - 2);
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(29));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(27));
    }
}

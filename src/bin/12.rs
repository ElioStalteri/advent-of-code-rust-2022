use console::Term;
use itertools::Itertools;
use rgb::*;
use std::{
    cmp::max,
    hash::{Hash, Hasher},
};
use textplots::{Chart, ColorPlot, Plot, Shape};

fn print_path(
    term: &Term,
    routes: &Vec<Vec<(usize, usize)>>,
    map: &Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
) {
    let max_x = map[0].len() as f32;
    let max_y = map.len() as f32;

    term.move_cursor_to(0, 0).unwrap();

    let offset = 300.0;

    Chart::new_with_y_range(
        180,
        90,
        0.0 - 20.0 + offset,
        max_x + 20.0 + offset,
        0.0 - 20.0 + offset,
        max_y + 20.0 + offset,
    )
    .linecolorplot(
        &Shape::Points(&[(start.1 as f32 + offset, start.0 as f32 + offset)]),
        RGB8 {
            r: 0,
            g: 255_u8,
            b: 0,
        },
    )
    .linecolorplot(
        &Shape::Points(&[(end.1 as f32 + offset, end.0 as f32 + offset)]),
        RGB8 {
            r: 0,
            g: 0,
            b: 255_u8,
        },
    )
    .linecolorplot(
        &Shape::Points(
            routes
                .iter()
                .flatten()
                .map(|c| (offset + c.1 as f32, offset + c.0 as f32))
                .collect_vec()
                .as_slice(),
        ),
        RGB8 {
            r: 255_u8,
            g: 0,
            b: 0,
        },
    )
    .nice();

    std::thread::sleep(std::time::Duration::from_millis(20));
}

fn find_possible_routes(
    cur: (usize, usize),
    map: &Vec<Vec<i32>>,
    visited: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let (y, x) = cur;
    let mut checks: Vec<(i32, i32)> = vec![];
    if y != 0 {
        checks.push((-1, 0));
    }
    if y != map.len() - 1 {
        checks.push((1, 0));
    }
    if x != 0 {
        checks.push((0, -1));
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
        if delta_step < 2 {
            res.push(((y as i32 + dy) as usize, (x as i32 + dx) as usize))
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut initial: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
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
                        (b'a' - 1) as i32
                    } else if c == 'E' {
                        end = (y, x);
                        (b'z' + 1) as i32
                    } else {
                        (c as u8) as i32
                    }
                })
                .collect()
        })
        .collect();

    let term = console::Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();

    let mut history_routes: Vec<Vec<(usize, usize)>> = vec![];
    let mut completed_routes: Vec<Vec<(usize, usize)>> = vec![];
    let mut routes: Vec<Vec<(usize, usize)>> = vec![vec![initial]];
    while !routes.is_empty() {
        let mut tmp: Vec<Vec<(usize, usize)>> = vec![];
        for (i, r) in routes.clone().iter().enumerate() {
            let curr = *r.last().unwrap();
            let found = find_possible_routes(curr, &map, r);
            let mut new_routes: Vec<Vec<(usize, usize)>> = vec![];
            for f in &found {
                let mut current_route = r.clone();
                current_route.push(*f);
                if *f == end {
                    completed_routes.push(current_route);
                } else {
                    new_routes.push(current_route);
                }
            }

            tmp.splice((tmp.len()).., new_routes);
        }
        tmp.sort_by(|a, b| {
            let last_a = a.last().unwrap();
            let last_b = b.last().unwrap();
            (last_b.1 as i32 - end.1 as i32)
                .abs()
                .partial_cmp(&(last_a.1 as i32 - end.1 as i32).abs())
                .unwrap()
        });

        if !tmp.is_empty() {
            let max_slice = if tmp.len() > 10 { 
                history_routes.splice(history_routes.len().., tmp[10..].to_vec());
                10 
            } else { tmp.len() };
            routes = tmp[0..max_slice].to_vec();
        }else{
            routes = history_routes.splice(0..10,[]).collect();
        }

        print_path(&term, &routes, &map, initial, end);
    }

    // Some(completed_routes.iter().map(|r| r.len()).min().unwrap() as i32 - 1)
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}

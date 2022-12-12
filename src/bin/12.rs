use console::Term;
use itertools::Itertools;
use pathfinding::{directed::dijkstra, *};
use rgb::*;
use std::{
    cmp::max,
    collections::HashMap,
    hash::{Hash, Hasher},
};
use textplots::{Chart, ColorPlot, Plot, Shape};

fn print_path(
    term: &Term,
    routes: &Vec<(usize, usize)>,
    map: &Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
    show_line: bool,
) {
    let max_x = map[0].len() as f32;
    let max_y = map.len() as f32;

    term.move_cursor_to(0, 0).unwrap();

    let offset = 300.0;

    let routes_formatted = routes
        .iter()
        .map(|c| (offset + c.1 as f32, offset + c.0 as f32))
        .collect_vec();
    let data: Shape;
    if show_line {
        data = Shape::Lines(routes_formatted.as_slice());
    } else {
        data = Shape::Points(routes_formatted.as_slice())
    }

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
        &data,
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

    let result = dijkstra::dijkstra(
        &initial,
        |cur| {
            find_possible_routes(*cur, &map, &vec![])
                .into_iter()
                .map(|p| (p, 1))
        },
        |&p| p == end,
    );

    Some(result.unwrap().1)

    // let term = console::Term::stdout();
    // term.hide_cursor().unwrap();
    // term.clear_screen().unwrap();

    // let mut completed_routes: HashMap<String, Vec<(usize, usize)>> = HashMap::from([(
    //     "".to_string() + &initial.0.to_string() + "," + &initial.1.to_string(),
    //     vec![initial],
    // )]);

    // let mut edges: Vec<(usize, usize)> = vec![initial];
    // let mut visited: Vec<(usize, usize)> = vec![initial];
    // loop {
    //     let mut new_completed_routes: HashMap<String, Vec<(usize, usize)>> = HashMap::from([]);
    //     let mut new_edges: Vec<Vec<(usize, usize)>> = vec![];
    //     for edge in edges.clone() {
    //         let found = find_possible_routes(edge, &map, &visited);
    //         new_edges.push(found.clone());
    //         let prev_key = "".to_string() + &edge.0.to_string() + "," + &edge.1.to_string();
    //         for p in found {
    //             let new_key = "".to_string() + &p.0.to_string() + "," + &p.1.to_string();
    //             let mut new_value = completed_routes.get(&prev_key).unwrap_or(&vec![]).clone();
    //             new_value.push(p);
    //             if completed_routes.get(&new_key).is_some() {
    //                 if completed_routes.get(&new_key).unwrap().len() > new_value.len() {
    //                     new_completed_routes.entry(new_key).or_insert(new_value);
    //                 }
    //             } else {
    //                 new_completed_routes.entry(new_key).or_insert(new_value);
    //             }
    //         }
    //     }
    //     completed_routes = new_completed_routes;

    //     let formatted: Vec<(usize, usize)> = new_edges
    //         .iter()
    //         .flatten()
    //         .unique()
    //         .map(|(y, x)| (*y, *x))
    //         .collect();

    //     if formatted.iter().any(|p| p == &end) {
    //         dbg!("found");
    //         break;
    //     }

    //     let mut tmp: Vec<Vec<(usize, usize)>> = vec![visited];
    //     tmp.push(formatted.clone());
    //     visited = tmp.iter().flatten().map(|(y, x)| (*y, *x)).collect();
    //     edges = formatted;
    //     print_path(&term, &visited, &map, initial, end, false);
    // }

    // let end_key = "".to_string() + &end.0.to_string() + "," + &end.1.to_string();
    // print_path(
    //     &term,
    //     &completed_routes.get(&end_key).unwrap(),
    //     &map,
    //     initial,
    //     end,
    //     true,
    // );
    // dbg!(completed_routes.get(&end_key).unwrap().len());

    // let mut history_routes: Vec<Vec<(usize, usize)>> = vec![];
    // let mut completed_routes: Vec<Vec<(usize, usize)>> = vec![];
    // let mut routes: Vec<Vec<(usize, usize)>> = vec![vec![initial]];
    // while !routes.is_empty() {
    //     let mut tmp: Vec<Vec<(usize, usize)>> = vec![];
    //     for (i, r) in routes.clone().iter().enumerate() {
    //         let curr = *r.last().unwrap();
    //         let found = find_possible_routes(curr, &map, r);
    //         let mut new_routes: Vec<Vec<(usize, usize)>> = vec![];
    //         for f in &found {
    //             let mut current_route = r.clone();
    //             current_route.push(*f);
    //             if *f == end {
    //                 completed_routes.push(current_route);
    //             } else {
    //                 new_routes.push(current_route);
    //             }
    //         }

    //         tmp.splice((tmp.len()).., new_routes);
    //     }
    //     tmp.sort_by(|a, b| {
    //         let last_a = a.last().unwrap();
    //         let last_b = b.last().unwrap();
    //         (last_b.1 as i32 - end.1 as i32)
    //             .abs()
    //             .partial_cmp(&(last_a.1 as i32 - end.1 as i32).abs())
    //             .unwrap()
    //     });

    //     if !tmp.is_empty() {
    //         let max_slice = if tmp.len() > 10 {
    //             history_routes.splice(history_routes.len().., tmp[10..].to_vec());
    //             10
    //         } else { tmp.len() };
    //         routes = tmp[0..max_slice].to_vec();
    //     }else{
    //         routes = history_routes.splice(0..10,[]).collect();
    //     }

    //     print_path(&term, &routes, &map, initial, end);
    // }

    // Some(completed_routes.iter().map(|r| r.len()).min().unwrap() as i32 - 1)
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

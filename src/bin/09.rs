use itertools::Itertools;
use rgb::*;
use std::{
    cmp::max,
    hash::{Hash, Hasher},
};
use textplots::{Chart, ColorPlot, Plot, Shape};

#[derive(Debug, Default, Clone)]
struct Movement<'a> {
    movement: &'a str,
    repeat: i32,
}

#[derive(Debug, Default, Clone, Eq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Coord {
    pub fn follow(&mut self, o: &Coord, history: Option<&mut Vec<Coord>>) -> Coord {
        let dist_x = o.x - self.x;
        let dist_y = o.y - self.y;
        if dist_x.abs() > 1 && dist_y.abs() > 1{
            self.x += dist_x - dist_x.signum();
            self.y += dist_y - dist_y.signum();
            if history.is_some() {
                history.unwrap().push(Coord {
                    x: self.x,
                    y: self.y,
                });
            }
        } else if dist_x.abs() > 1 {
            self.x += dist_x - dist_x.signum();
            self.y = o.y;
            if history.is_some() {
                history.unwrap().push(Coord {
                    x: self.x,
                    y: self.y,
                });
            }
        } else if dist_y.abs() > 1 {
            self.x = o.x;
            self.y += dist_y - dist_y.signum();
            if history.is_some() {
                history.unwrap().push(Coord {
                    x: self.x,
                    y: self.y,
                });
            }
        }
        self.clone()
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let movements: Vec<Movement> = input
        .split('\n')
        .map(|v| {
            let components: Vec<&str> = v.split(' ').into_iter().collect_vec();

            Movement {
                movement: components[0],
                repeat: components[1].parse::<i32>().ok().unwrap(),
            }
        })
        .collect();

    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };
    let mut history = vec![head.clone()];
    for m in movements {
        for _ in 0..m.repeat {
            match m.movement {
                "U" => {
                    head.y += 1;
                }
                "D" => {
                    head.y -= 1;
                }
                "R" => {
                    head.x += 1;
                }
                "L" => {
                    head.x -= 1;
                }
                _ => {}
            }
            tail = tail.follow(&head, Some(&mut history));
        }
    }

    Some(history.iter().unique().count() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let movements: Vec<Movement> = input
        .split('\n')
        .map(|v| {
            let components: Vec<&str> = v.split(' ').into_iter().collect_vec();

            Movement {
                movement: components[0],
                repeat: components[1].parse::<i32>().ok().unwrap(),
            }
        })
        .collect();
    let number_of_coords = 10;

    let mut coords: Vec<Coord> = vec![];
    for _ in 0..number_of_coords {
        coords.push(Coord { x: 0, y: 0 });
    }

    let term = console::Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();


    let mut history = vec![Coord { x: 0, y: 0 }];
    for m in movements {
        for _ in 0..m.repeat {
            match m.movement {
                "U" => {
                    coords[0].y += 1;
                }
                "D" => {
                    coords[0].y -= 1;
                }
                "R" => {
                    coords[0].x += 1;
                }
                "L" => {
                    coords[0].x -= 1;
                }
                _ => {}
            }
            for (i, _) in coords.clone().iter().enumerate() {
                if i != 0 {
                    let prev_c = coords[i - 1].clone();
                    if i == (coords.len() - 1) {
                        coords[i] = coords[i].follow(&prev_c, Some(&mut history));
                    } else {
                        coords[i] = coords[i].follow(&prev_c, None);
                    }
                }
            }
        }
    
        term.move_cursor_to(0, 0).unwrap();
        Chart::new_with_y_range(180, 60, -200.0 + 300.0, 100.0 + 300.0, 200.0 + -200.0,200.0+70.0)
        .linecolorplot(
            &Shape::Lines(
                coords
                    .iter()
                    .map(|c| (300.0 + c.x as f32, 200.0 + c.y as f32))
                    .collect_vec()
                    .as_slice(),
            ),
            RGB8 {
                r: 255_u8,
                g: 0,
                b: 0,
            },
        ).nice();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    // 2743 too high
    // 2531 too low
    Chart::new(180, 60, -300.0, 300.0)
        .linecolorplot(
            &Shape::Lines(
                history
                    .iter()
                    .map(|c| (c.x as f32, c.y as f32))
                    .collect_vec()
                    .as_slice(),
            ),
            RGB8 {
                r: 255_u8,
                g: 0,
                b: 0,
            },
        )
        .display();

        
    Some(history.iter().unique().count() as i32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(input), Some(36));
    }
}

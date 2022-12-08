use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let trees: Vec<Vec<i32>> = input
        .split('\n')
        .map(|v| {
            v.chars()
                .map(|v| v.to_string().parse::<i32>().ok().unwrap())
                .collect()
        })
        .collect();

    let outside_trees = ((trees.len() + trees[0].len() - 2) * 2) as i32;

    let mut max = -1;
    let visible_from_left: Vec<Vec<i32>> = trees
        .iter()
        .map(|row| {
            max = -1;
            row.iter()
                .map(|v| {
                    if v > &max {
                        max = *v;
                        return *v;
                    }
                    -1
                })
                .collect()
        })
        .collect();

    max = -1;
    let visible_from_right: Vec<Vec<i32>> = trees
        .iter()
        .map(|r| {
            max = -1;
            let mut row = r.clone();
            row.reverse();
            row = row
                .iter()
                .map(|v| {
                    if v > &max {
                        max = *v;
                        return *v;
                    }
                    -1
                })
                .collect();
            row.reverse();
            row
        })
        .collect();

    let mut max: Vec<i32> = vec![-1; trees[0].len()];
    let visible_from_top: Vec<Vec<i32>> = trees
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, v)| {
                    if v > &max[i] {
                        max[i] = *v;
                        return *v;
                    }
                    -1
                })
                .collect()
        })
        .collect();

    let mut max: Vec<i32> = vec![-1; trees[0].len()];
    let visible_from_bottom: Vec<Vec<i32>> = {
        let mut tmp: Vec<Vec<i32>> = trees.clone();
        tmp.reverse();
        tmp = tmp
            .iter()
            .map(|row| {
                // let mut row = r.clone();
                // row.reverse();
                row.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if v > &max[i] {
                            max[i] = *v;
                            return *v;
                        }
                        -1
                    })
                    .collect()
            })
            .collect();
        tmp.reverse();
        tmp
    };

    let mut count_visibles_coord: Vec<Vec<usize>> = vec![];
    let mut count_visibles = 0;
    for (ir, row) in trees.iter().enumerate() {
        for (ic, v) in row.iter().enumerate() {
            if ic != 0
                && ic != row.len() - 1
                && ir != 0
                && ir != trees.len() - 1
                && (visible_from_right[ir][ic] > -1
                    || visible_from_left[ir][ic] > -1
                    || visible_from_top[ir][ic] > -1
                    || visible_from_bottom[ir][ic] > -1)
            {
                count_visibles += 1;
                count_visibles_coord.push(vec![ir, ic]);
            }
        }
    }

    Some(count_visibles + outside_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees: Vec<Vec<i32>> = input
        .split('\n')
        .map(|v| {
            v.chars()
                .map(|v| v.to_string().parse::<i32>().ok().unwrap())
                .collect()
        })
        .collect();

    let mut max: Vec<i32> = vec![];
    let visible_from_left: Vec<Vec<i32>> = trees
        .iter()
        .map(|row| {
            max = vec![];
            row.iter()
                .map(|v| {
                    if max.is_empty() {
                        max.push(*v);
                        return 0;
                    }
                    let mut count = 0;
                    for b in max.iter().rev() {
                        count += 1;
                        if b >= v {
                            break;
                        }
                    }
                    max.push(*v);
                    count
                })
                .collect()
        })
        .collect();

    max = vec![];
    let visible_from_right: Vec<Vec<i32>> = trees
        .iter()
        .map(|r| {
            max = vec![];
            let mut row = r.clone();
            row.reverse();
            row = row
                .iter()
                .map(|v| {
                    if max.is_empty() {
                        max.push(*v);
                        return 0;
                    }
                    let mut count = 0;
                    for b in max.iter().rev() {
                        count += 1;
                        if b >= v {
                            break;
                        }
                    }
                    max.push(*v);
                    count
                })
                .collect();
            row.reverse();
            row
        })
        .collect();


    let mut max: Vec<Vec<i32>> = vec![vec![]; trees[0].len()];
    let visible_from_top: Vec<Vec<i32>> = trees
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, v)| {
                    if max[i].is_empty() {
                        max[i].push(*v);
                        return 0;
                    }
                    let mut count = 0;
                    for b in max[i].iter().rev() {
                        count += 1;
                        if b >= v {
                            break;
                        }
                    }
                    max[i].push(*v);
                    count
                })
                .collect()
        })
        .collect();

    

    let mut max: Vec<Vec<i32>> = vec![vec![]; trees[0].len()];
    let visible_from_bottom: Vec<Vec<i32>> = {
        let mut tmp: Vec<Vec<i32>> = trees.clone();
        tmp.reverse();
        tmp = tmp
            .iter()
            .map(|row| {
                // let mut row = r.clone();
                // row.reverse();
                row.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if max[i].is_empty() {
                            max[i].push(*v);
                            return 0;
                        }
                        let mut count = 0;
                        for b in max[i].iter().rev() {
                            count += 1;
                            if b >= v {
                                break;
                            }
                        }
                        max[i].push(*v);
                        count
                    })
                    .collect()
            })
            .collect();
        tmp.reverse();
        tmp
    };



    // let mut count_visibles_coord: Vec<Vec<usize>> = vec![];
    // let mut count_visibles = 0;
    // for (ir, row) in trees.iter().enumerate() {
    //     for (ic, v) in row.iter().enumerate() {
    //         if ic != 0
    //             && ic != row.len() - 1
    //             && ir != 0
    //             && ir != trees.len() - 1
    //             && (visible_from_right[ir][ic] > -1
    //                 || visible_from_left[ir][ic] > -1
    //                 || visible_from_top[ir][ic] > -1
    //                 || visible_from_bottom[ir][ic] > -1)
    //         {
    //             count_visibles += 1;
    //             count_visibles_coord.push(vec![ir, ic]);
    //         }
    //     }
    // }

    None
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}

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

    let outside_trees = (trees.len() + trees[0].len()) as i32;

    let mut max = -1;
    let visible_from_right: Vec<Vec<i32>> = trees
        .iter()
        .map(|row| {
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
    let visible_from_left: Vec<Vec<i32>> = trees
        .iter()
        .map(|r| {
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
    let mut count_visibles = 0 ;
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
                count_visibles +=1;
                count_visibles_coord.push(vec![ir,ic]);
            }
        }
    }
    dbg!(count_visibles_coord);
    Some(count_visibles + outside_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
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

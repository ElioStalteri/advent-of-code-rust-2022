use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    // A X --> Rock -> 1 point
    // B Y --> paper -> 2 point
    // C Z --> scissors -> 3 point
    // wins:
    // s -> p -> r -> s
    // 0 lost
    // 3 draw
    // 6 won

    let win_map = HashMap::from([("s", "p"), ("p", "r"), ("r", "s")]);

    let player1 = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);

    let player2 = HashMap::from([("X", "r"), ("Y", "p"), ("Z", "s")]);

    let choise_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);

    let games = input.split("\n");

    let mut points = 0;
    for game in games {
        let choises = game.split(" ").map(|v| v.trim()).collect::<Vec<&str>>();
        if choises.len() > 1 {
            let player1_choise = player1.get(choises[0]).unwrap();
            let player2_choise = player2.get(choises[1]).unwrap();

            if player1_choise == player2_choise {
                points += 3
            } else if win_map.get(player2_choise).unwrap() == player1_choise {
                points += 6
            }

            let player2_choise_points = choise_points.get(player2_choise).unwrap();
            points += player2_choise_points
        }
    }

    Some(points)
}

pub fn part_two(input: &str) -> Option<i32> {
    // A --> Rock -> 1 point
    // B --> paper -> 2 point
    // C --> scissors -> 3 point
    // wins:
    // s -> p -> r -> s
    // X -> 0 lost
    // Y -> 3 draw
    // Z -> 6 won

    let loose_map = HashMap::from([("s", "p"), ("p", "r"), ("r", "s")]);

    let win_map = HashMap::from([("s", "r"), ("p", "s"), ("r", "p")]);

    let player1 = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);

    let outcomes = HashMap::from([("X", "l"), ("Y", "d"), ("Z", "w")]);

    let choise_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);

    let games = input.split("\n");

    let mut points = 0;
    for game in games {
        let choises = game.split(" ").map(|v| v.trim()).collect::<Vec<&str>>();
        if choises.len() > 1 {
            let player1_choise = player1.get(choises[0]).unwrap();
            let outcome = *outcomes.get(choises[1]).unwrap();

            match outcome {
                "l" => {
                    let player2_choise = *loose_map.get(player1_choise).unwrap();
                    let c_point = choise_points.get(player2_choise).unwrap();
                    points += c_point ;
                }
                "d" => {
                    let c_point = choise_points.get(player1_choise).unwrap();
                    points += c_point + 3;
                }
                "w" => {
                    let player2_choise = *win_map.get(player1_choise).unwrap();
                    let c_point = choise_points.get(player2_choise).unwrap();
                    points += c_point + 6;
                }
                _ => {}
            }
        }
    }

    Some(points)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

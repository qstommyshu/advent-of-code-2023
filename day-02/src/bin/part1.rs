fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn new(s: &str) -> Draw {
        s.split(", ").fold(
            Draw {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, item| {
                let (num, color) = item.split_once(" ").unwrap();
                let num = num.parse().unwrap();
                match color {
                    "red" => {
                        acc.red = num;
                    }
                    "green" => {
                        acc.green = num;
                    }
                    "blue" => {
                        acc.blue = num;
                    }
                    _ => panic!("at the disco"),
                }
                acc
            }
        )
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .enumerate()
        .map(
            /* turn a line into a game */
            |(idx, line)| {
                let (_, draws) = line.split_once(": ").unwrap();
                let draws = draws.split("; ").map(Draw::new).collect();
                Game { id: idx + 1, draws }
            }
        )
        .collect()
    // .filter(/* filter out impossible game */)
    // .map(/* get id of remaining games */)
    // .sum()
}

fn part1(input: &str) -> String {
    // Goal: group Games, Games contains Draws
    // 1. split input into lines
    // 2. each line, parse to get three draws
    // Now we get Games{Draws{RGB}}
    // then check with 12R, 13G, 14B
    let games = parse_input(input);
    games
        .into_iter()
        .filter(|game| {
            // filter out games where game is not possible
            game.draws.iter().all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8".to_string(), part1(input));
    }
}

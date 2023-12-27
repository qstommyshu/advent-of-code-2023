use std::iter::zip;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// TODO: parse to struct is a good idea
struct Race {
    time: u32,
    dist: u32,
}

fn part1(input: &str) -> String {
    // TODO: split_once split to an option tuple
    let (time, dist) = input.split_once("\n").unwrap();
    let time: Vec<u32> = time
        .split_whitespace()
        .filter_map(|num: &str| { num.parse::<u32>().ok() })
        .collect();
    let dist: Vec<u32> = dist
        .split_whitespace()
        .filter_map(|num| { num.parse::<u32>().ok() })
        .collect();
    let races = zip(time, dist).map(|(time, dist)| Race { time, dist });
    races
        .map(|race| {
            // TODO: for loop map interesting
            (0..=race.time)
                .map(|elapsed| {
                    let speed = elapsed;
                    speed * (race.time - elapsed)
                })
                .filter(|&dist| dist > race.dist)
                .count()
        })
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!("288".to_string(), part1(input));
    }
}

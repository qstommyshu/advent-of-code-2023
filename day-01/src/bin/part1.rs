fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let output = input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .map(|line| {
            let mut it = line.chars().filter_map(|character| { character.to_digit(10) });
            let first = it.next().expect("should be a number");

            (
                match it.last() {
                    Some(last) => format!("{first}{last}"),
                    None => format!("{first}{first}"),
                }
            )
                .parse::<u32>()
                .expect("Should be a valid number")
        })
        .sum::<u32>();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142".to_string(), part1(input));
    }
}

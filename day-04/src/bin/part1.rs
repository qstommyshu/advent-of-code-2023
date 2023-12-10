fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|card| {
            let (_, nums) = card.split_once(": ").unwrap();
            let (win_nums, own_nums) = nums.split_once(" | ").unwrap();
            let win_nums = win_nums.split_whitespace().collect::<Vec<_>>();
            let own_nums = own_nums.split_whitespace().collect::<Vec<_>>();
            // can use fold here
            let match_num = own_nums
                .iter()
                .filter(|num| win_nums.contains(num))
                .count();

            let score = if match_num > 0 { (2_u32).pow((match_num as u32) - 1) } else { 0 };
            score
            // dbg!(score)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13".to_string(), part1(input));
    }
}

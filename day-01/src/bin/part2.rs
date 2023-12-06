fn main() {
    let input = include_str!("day1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            10 * vec.first().expect("Every line must have at least one digit")
                + vec.last().expect("Every line must have at least one digit")
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = solution(input);
        let expected = 281;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part() {
        let input = include_str!("day1.txt");
        let result = solution(input);
        let expected = 53389;
        assert_eq!(result, expected);
    }
}

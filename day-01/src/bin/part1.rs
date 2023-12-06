fn main() {
    let input = include_str!("day1.txt");
    let output = solution(input);
    dbg!(output);
}


fn solution(input: &str) -> u32 {
    input.lines()
        .map(|line|
            line.chars()
                .filter(|char| char.is_digit(10))
                .map(|digit| digit.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        )
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = solution(input);
        let expected = 142;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution() {
        let input = include_str!("day1.txt");
        let result = solution(input);
        let expected = 54338;
        assert_eq!(result, expected);
    }
}


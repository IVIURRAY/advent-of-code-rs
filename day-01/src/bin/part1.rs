fn main() {
    let input = include_str!("day1.txt");
    let output = solution(input);
    dbg!(output);
}


fn solution(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        let first_char = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last_char = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

        let combined= format!("{}{}", first_char, last_char).parse::<i32>().unwrap();
        total += combined;
    }

    total
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


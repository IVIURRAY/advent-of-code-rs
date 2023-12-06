fn main() {
    let input = include_str!("day4.txt");
    let output = solution(input);
    dbg!(output);
}


fn solution(input: &str) -> u32 {
    input.lines().map(|line| {
        let game_id = &line[5..line.find(":").unwrap()];
        line[line.find(":").unwrap() + 2..line.len()].lines()
            .map(|t| {
                let number: Vec<&str> = t.split("|").collect();
                let winners = number.first().unwrap().split_whitespace();
                let my_numbers: Vec<&str>= number.last().unwrap().split_whitespace().collect();
                // println!("winners: {:?} numbers: {:?}", winners, my_numbers);
                let matches: Vec<&str> = winners
                    .filter(|x| my_numbers.contains(x)).collect();
                //
                let result = if matches.is_empty() { 0 } else { 2_u32.pow(matches.len() as u32 - 1) };
                println!("{} {} {:?}", game_id, result, matches);
                result
            }).sum::<u32>()
    }).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = solution(input);
        let expected = 13;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part() {
        let input = include_str!("day4.txt");
        let result = solution(input);
        let expected = 21919;
        assert_eq!(result, expected);
    }
}


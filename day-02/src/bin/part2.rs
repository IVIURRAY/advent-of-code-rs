fn main() {
    let input = include_str!("day2.txt");
    let output = solution(input);
    dbg!(output);
}



fn solution(input: &str) -> u32 {
    input.lines().map(|line| {
        let game_id = line[5..line.find(":").unwrap()].parse::<u32>().unwrap();
        println!("{}", game_id);
        let game_content = line[line.find(":").unwrap() + 2..line.len()].split("; ")
            .flat_map(|cubes|
                cubes.split(", ")
                    .map(|pair| {
                        let hand: Vec<&str> = pair.split(" ").collect();
                        let number = hand.first().unwrap();
                        let colour = hand.last().unwrap();

                        (number.clone(), colour.clone())
                    })
            )
            .collect::<Vec<(&str, &str)>>();

        let red = game_content.iter().filter(|game| game.1 == "red").map(|g| g.0.parse::<u32>().unwrap()).max().unwrap_or(0);
        let blue = game_content.iter().filter(|game| game.1 == "blue").map(|g| g.0.parse::<u32>().unwrap()).max().unwrap_or(0);
        let green = game_content.iter().filter(|game| game.1 == "green").map(|g| g.0.parse::<u32>().unwrap()).max().unwrap_or(0);

        println!("{} {} {}", red, blue, green);
        red * blue * green
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = solution(input);
        let expected = 2286;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part() {
        let input = include_str!("day2.txt");
        let result = solution(input);
        let expected = 63711;
        assert_eq!(result, expected);
    }
}


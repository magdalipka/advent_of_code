#[derive(Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn from(line: &str) -> Self {
        let mut round = Round::default();
        let data = line.split(", ").collect::<Vec<_>>();
        let mut index = 0;
        while index < data.len() {
            match data[index].split(" ").collect::<Vec<_>>()[..] {
                [x, "red"] => round.red = x.parse::<u32>().unwrap(),
                [x, "green"] => round.green = x.parse::<u32>().unwrap(),
                [x, "blue"] => round.blue = x.parse::<u32>().unwrap(),

                _ => panic!("Unrecognized color!"),
            }
            index += 1;
        }
        return round;
    }
}

#[derive(Default)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from(line: &str) -> Self {
        let mut game = Game::default();
        let data = line.split(": ").collect::<Vec<&str>>();
        let id = data[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        game.id = id;
        game.rounds = data[1]
            .split("; ")
            .map(|round| Round::from(round))
            .collect::<Vec<_>>();
        return game;
    }
}

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from(line))
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process(input), 8);
    }
}

use std::cmp;

#[derive(Default)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn from(line: &str) -> Self {
        let mut game = Game::default();
        let data = line.split(": ").collect::<Vec<&str>>();
        let id = data[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        game.id = id;

        data[1].split("; ").for_each(|round| {
            let line_data = round.split(", ").collect::<Vec<_>>();
            let mut index = 0;
            while index < line_data.len() {
                match line_data[index].split(" ").collect::<Vec<_>>()[..] {
                    [x, "red"] => game.red = cmp::max(game.red, x.parse::<u32>().unwrap()),
                    [x, "green"] => game.green = cmp::max(game.green, x.parse::<u32>().unwrap()),
                    [x, "blue"] => game.blue = cmp::max(game.blue, x.parse::<u32>().unwrap()),

                    _ => panic!("Unrecognized color!"),
                }
                index += 1;
            }
        });

        return game;
    }
}

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from(line))
        .map(|game| game.red * game.green * game.blue)
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
        assert_eq!(process(input), 2286);
    }
}

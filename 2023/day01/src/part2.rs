pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut res: Vec<u32> = [].to_vec();
            let mut index = 0;
            while index < line.len() {
                let split_line = &line[index..];
                let vec_line = split_line.chars().collect::<Vec<char>>();
                if vec_line[0].is_digit(10) {
                    res.push(vec_line[0].to_digit(10).unwrap());
                } else if split_line.starts_with("one") {
                    res.push(1);
                } else if split_line.starts_with("two") {
                    res.push(2);
                } else if split_line.starts_with("three") {
                    res.push(3);
                } else if split_line.starts_with("four") {
                    res.push(4);
                } else if split_line.starts_with("five") {
                    res.push(5);
                } else if split_line.starts_with("six") {
                    res.push(6);
                } else if split_line.starts_with("seven") {
                    res.push(7);
                } else if split_line.starts_with("eight") {
                    res.push(8);
                } else if split_line.starts_with("nine") {
                    res.push(9);
                }
                index += 1;
            }

            return res;
        })
        .map(|line| (10 * line.first().unwrap()) + line.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process(input), 281);
    }
}

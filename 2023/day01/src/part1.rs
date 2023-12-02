pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
        })
        .map(|line| line.collect::<Vec<_>>())
        .map(|line| (10 * line.first().unwrap()) + line.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(input), 142);
    }
}

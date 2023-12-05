pub fn process(input: &str) -> usize {
    let mut cards_count = input.lines().map(|_| 1).collect::<Vec<_>>();

    return input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            let numbers = line
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .collect::<Vec<_>>();

            let winning_numbers = numbers[0]
                .split(" ")
                .filter_map(|v| match v.parse::<u32>() {
                    Ok(x) => Some(x),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let our_numbers = numbers[1]
                .split(" ")
                .filter_map(|v| match v.parse::<u32>() {
                    Ok(x) => Some(x),
                    _ => None,
                })
                .filter(|v| winning_numbers.contains(v))
                .collect::<Vec<_>>();

            let mut i = 1;
            while i + line_number < cards_count.len() && i <= our_numbers.len() {
                cards_count[i + line_number] += cards_count[line_number];
                i += 1;
            }

            return cards_count[line_number];
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 30);
    }
}

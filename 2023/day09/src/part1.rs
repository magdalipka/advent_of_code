pub fn differences(seq: &Vec<i64>) -> Vec<i64> {
    let mut res = Vec::new();
    for (index, item) in seq.iter().enumerate().skip(1) {
        res.push(item - (seq.get(index - 1).unwrap()))
    }
    return res;
}

pub fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|seq| {
            let mut current = seq.to_owned();
            let mut res = Vec::new();
            while !current.iter().all(|item| *item == 0) {
                res.push(current.last().unwrap().to_owned());
                current = differences(&current);
            }
            res.reverse();
            return res;
        })
        .map(|seq| {
            seq.iter().fold(0, |acc, item| {
                return acc + item;
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input), 114);
    }
}

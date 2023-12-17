use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let mut lines = input
        .lines()
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();
    let instructions = lines.remove(0).chars().collect::<Vec<_>>();

    let mut items: HashMap<&str, (&str, &str)> = HashMap::default();

    lines.iter().for_each(|line| {
        let (name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        let left = left.strip_prefix("(").unwrap();
        let right = right.strip_suffix(")").unwrap();
        items.insert(name, (&left, &right));
    });

    let mut current = "AAA";
    let mut steps = 0;
    while current != "ZZZ" {
        for i in instructions.iter() {
            steps += 1;
            if *i == 'L' {
                current = items.get(current).unwrap().0
            } else {
                current = items.get(current).unwrap().1
            }
        }
    }

    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(input), 6);
    }
}

#[derive(Copy, Clone, Debug)]
enum Item {
    Symbol { pos: usize, value: char },
    Number { pos: (usize, usize), value: u32 },
}

fn gear_value(line_no: usize, item_pos: &usize, schematic: &Vec<Vec<Item>>) -> u32 {
    let mut part_numbers: Vec<&u32> = Vec::default();

    // println!("line no {}, item_pos {}", line_no, item_pos);

    for item in &schematic[line_no] {
        match item {
            Item::Number { pos, value } => {
                if *item_pos + 1 == pos.0 || *item_pos == pos.1 + 1 {
                    part_numbers.push(value);
                }
            }
            _ => (),
        }
    }
    if line_no != 0 {
        for item in &schematic[line_no - 1] {
            match item {
                Item::Number { pos, value } => {
                    if *item_pos + 1 >= pos.0 && *item_pos <= pos.1 + 1 {
                        part_numbers.push(value);
                    }
                }
                _ => (),
            }
        }
    }
    if line_no != &schematic.len() - 1 {
        for item in &schematic[line_no + 1] {
            match item {
                Item::Number { pos, value } => {
                    if *item_pos + 1 >= pos.0 && *item_pos <= pos.1 + 1 {
                        part_numbers.push(value);
                    }
                }
                _ => (),
            }
        }
    }

    if part_numbers.len() != 2 {
        return 0;
    } else {
        return part_numbers[0] * part_numbers[1];
    }
}

pub fn process(input: &str) -> u32 {
    let mut schematic: Vec<Vec<Item>> = Vec::default();

    for line in input.lines() {
        let mut line_vec = Vec::default();
        let line_iter = line.chars().collect::<Vec<_>>();
        let mut item_index = 0;
        'inner: while item_index < line.len() {
            let item = line_iter[item_index];

            if item.is_digit(10) {
                let mut item_value = (item.to_digit(10)).unwrap();
                let start_index = item_index;
                item_index += 1;
                while item_index < line.len() && line_iter[item_index].is_digit(10) {
                    item_value = item_value * 10 + line_iter[item_index].to_digit(10).unwrap();
                    item_index += 1;
                }

                line_vec.push(Item::Number {
                    pos: (start_index, item_index - 1),
                    value: item_value,
                });
            } else if item.is_ascii() && item != '.' {
                line_vec.push(Item::Symbol {
                    pos: item_index,
                    value: item,
                });
                item_index += 1;
            } else {
                item_index += 1;
            }
        }
        schematic.push(line_vec);
    }

    let mut sum = 0;

    for (line_index, line) in schematic.iter().enumerate() {
        for item in line {
            match item {
                Item::Symbol { pos, value: '*' } => {
                    sum += gear_value(line_index, pos, &schematic);
                }
                _ => (),
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), 467835);
    }
}

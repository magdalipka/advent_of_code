struct ConverterEntry {
    start_range: u64,
    end_range: u64,
    modifier: i64,
}

impl ConverterEntry {
    fn from(input: &str) -> Self {
        let values = input
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        ConverterEntry {
            start_range: values[1],
            end_range: values[1] + values[2],
            modifier: values[0] as i64 - values[1] as i64,
        }
    }
    fn convert(&self, value: &u64) -> u64 {
        (self.modifier + (*value as i64)).try_into().unwrap()
    }
}

struct SingleConverter {
    converters: Vec<ConverterEntry>,
}

impl SingleConverter {
    fn from(input: &str) -> Self {
        SingleConverter {
            converters: input
                .lines()
                .map(|line| ConverterEntry::from(line))
                .collect::<Vec<_>>(),
        }
    }
    fn convert(&self, value: &u64) -> u64 {
        match self
            .converters
            .iter()
            .find(|i| i.start_range <= (*value) && i.end_range >= (*value))
        {
            Some(converter) => converter.convert(value),
            None => *value,
        }
    }
}

pub fn process(input: &str) -> u64 {
    let (seed_lines, maps) = input.split_once("\n\n").unwrap();
    let seed_data = seed_lines
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|v| match v.parse::<u64>() {
            Ok(x) => Some(x),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut seeds = Vec::default();
    let mut i = 0;
    while i < seed_data.len() {
        println!(
            "appending seed data: {} {} ",
            seed_data[i],
            seed_data[i + 1]
        );
        seeds.append(&mut ((seed_data[i]..seed_data[i] + seed_data[i + 1]).collect::<Vec<_>>()));
        i += 2;
    }

    println!("seeds length: {}", seeds.len());

    for map_data in maps.split("\n\n") {
        let (_, data) = map_data.split_once("\n").unwrap();
        let converter = SingleConverter::from(data);
        println!("converting");

        seeds = seeds
            .iter()
            .map(|seed| converter.convert(seed))
            .collect::<Vec<_>>();
    }

    let min = seeds.iter().min().unwrap();
    return *min;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process(input), 46);
    }
}

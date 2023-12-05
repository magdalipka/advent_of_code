use std::cmp;

#[derive(Debug, Clone, Copy)]
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
    fn convert(&self, value: &(u64, u64)) -> (u64, u64) {
        (
            (cmp::max(self.start_range, value.0) as i64 + self.modifier) as u64,
            (cmp::min(self.end_range, value.1) as i64 + self.modifier) as u64,
        )
    }
}

struct SingleConverter {
    converters: Vec<ConverterEntry>,
}

impl SingleConverter {
    fn from(input: &str) -> Self {
        let mut input_converters = input
            .lines()
            .map(|line| ConverterEntry::from(line))
            .collect::<Vec<_>>();
        input_converters.sort_by(|a, b| a.start_range.cmp(&b.start_range));

        let mut all_converters: Vec<ConverterEntry> = vec![];

        if input_converters[0].start_range > 0 {
            all_converters.push(ConverterEntry {
                start_range: 0,
                end_range: input_converters[0].start_range - 1,
                modifier: 0,
            });
        }
        all_converters.push(input_converters[0]);

        let mut i = 0;
        while i < input_converters.len() - 1 {
            if input_converters[i].end_range + 1 < input_converters[i + 1].start_range - 1 {
                all_converters.push(ConverterEntry {
                    start_range: input_converters[i].end_range + 1,
                    end_range: input_converters[i + 1].start_range - 1,
                    modifier: 0,
                });
            }
            all_converters.push(input_converters[i + 1]);

            i += 1;
        }

        all_converters.push(ConverterEntry {
            start_range: input_converters.last().unwrap().end_range + 1,
            end_range: u64::MAX,
            modifier: 0,
        });

        SingleConverter {
            converters: all_converters,
        }
    }
    fn convert(&self, value: &(u64, u64)) -> Vec<(u64, u64)> {
        self.converters
            .iter()
            .filter(|converter| {
                (converter.start_range <= value.0 && converter.end_range >= value.0)
                    || (converter.start_range <= value.1 && converter.end_range >= value.1)
                    || (converter.start_range >= value.0 && converter.end_range <= value.1)
            })
            .map(|converter| converter.convert(value))
            .collect::<Vec<_>>()
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
    let mut seed_ranges: Vec<(u64, u64)> = Vec::default();
    let mut i = 0;
    while i < seed_data.len() {
        seed_ranges.push((seed_data[i], seed_data[i] + seed_data[i + 1]));
        i += 2;
    }

    for map_data in maps.split("\n\n") {
        let (_, data) = map_data.split_once("\n").unwrap();
        let converter = SingleConverter::from(data);

        seed_ranges = seed_ranges
            .iter()
            .flat_map(|seed| converter.convert(seed))
            .collect::<Vec<_>>();
    }

    let min = seed_ranges.iter().map(|(left, _)| left).min().unwrap();
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

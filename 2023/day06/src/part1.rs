use std::iter::zip;

// x        time to hold button, also speed
// t-x      time to run
// (t-x)*x  distance
// w        current record

// minimum to win:
// - x2 + t*x - w > 0
// x2 - t*x + w < 0

// sqrt(t^2-4w)

fn get_amount_of_winning_values(race: &(u32, u32)) -> u32 {
    let delta = f32::sqrt((race.0 * race.0 - 4 * race.1) as f32);
    let left = ((race.0 as f32 - delta) / 2.0).floor() as u32 + 1;
    let right = ((race.0 as f32 + delta) / 2.0).ceil() as u32 - 1;

    right - left + 1
}

pub fn process(input: &str) -> u32 {
    let races = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|v| match v.parse::<u32>() {
                    Ok(x) => Some(x),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let race_times = races.first().unwrap();
    let record_times = races.last().unwrap();
    let races = zip(race_times.to_owned(), record_times.to_owned());

    return races
        .map(|r| get_amount_of_winning_values(&r))
        .fold(1, |acc, item| acc * item);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input), 288);
    }
}

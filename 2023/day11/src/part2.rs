pub fn process(input: &str) -> usize {
    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().len();

    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.char_indices()
                .filter_map(move |(char_index, c)| match c {
                    '#' => Some((line_index, char_index)),
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    // get all empty space
    let empty_rows = (0..rows)
        .filter(|row| !galaxies.iter().any(|g| g.0 == *row))
        .collect::<Vec<_>>();
    let empty_cols = (0..columns)
        .filter(|col| !galaxies.iter().any(|g| g.1 == *col))
        .collect::<Vec<_>>();

    // expand empty space
    for g in galaxies.iter_mut() {
        g.0 += empty_rows.iter().filter(|row| g.0 > **row).count() * 999999;
        g.1 += empty_cols.iter().filter(|col| g.1 > **col).count() * 999999;
    }

    // count the distances
    let mut sum_of_distances = 0;
    for source_index in 0..galaxies.len() {
        for destination_index in (source_index + 1)..galaxies.len() {
            sum_of_distances += (galaxies[source_index].0).abs_diff(galaxies[destination_index].0)
                + galaxies[source_index]
                    .1
                    .abs_diff(galaxies[destination_index].1);
        }
    }

    return sum_of_distances;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(process(input), 82000210);
    }
}

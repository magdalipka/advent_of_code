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
        g.0 += empty_rows.iter().filter(|row| g.0 > **row).count();
        g.1 += empty_cols.iter().filter(|col| g.1 > **col).count();
    }

    // count the distances
    let mut sum_of_distances = 0;
    for source in galaxies.iter() {
        for destination in galaxies.iter() {
            sum_of_distances +=
                (source.0).abs_diff(destination.0) + source.1.abs_diff(destination.1);
        }
    }

    return sum_of_distances / 2;
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
        assert_eq!(process(input), 374);
    }
}

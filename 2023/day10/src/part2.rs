fn connects_up(c: &char) -> bool {
    *c == '|' || *c == 'L' || *c == 'J' || *c == 'S'
}
fn connects_down(c: &char) -> bool {
    *c == '|' || *c == '7' || *c == 'F' || *c == 'S'
}
fn connects_left(c: &char) -> bool {
    *c == '-' || *c == '7' || *c == 'J' || *c == 'S'
}
fn connects_right(c: &char) -> bool {
    *c == '-' || *c == 'L' || *c == 'F' || *c == 'S'
}

fn color_pipe(
    pipes: &mut Vec<Vec<(char, usize)>>,
    start_position: (usize, usize),
    prev_position: (usize, usize),
    color: usize,
) -> Option<usize> {
    let (mut current_y, mut current_x) = start_position;
    let mut prev_position = prev_position.to_owned();
    return loop {
        if pipes[current_y][current_x].0 == 'S' {
            pipes[current_y][current_x].1 = color;
            break Some(color);
        } else if current_y > 0
            && prev_position != (current_y - 1, current_x)
            && connects_up(&pipes[current_y][current_x].0)
            && connects_down(&pipes[current_y - 1][current_x].0)
        {
            prev_position = (current_y, current_x);
            pipes[current_y][current_x].1 = color;
            current_y -= 1;
        } else if current_y < pipes.len() - 1
            && prev_position != (current_y + 1, current_x)
            && connects_down(&pipes[current_y][current_x].0)
            && connects_up(&pipes[current_y + 1][current_x].0)
        {
            prev_position = (current_y, current_x);
            pipes[current_y][current_x].1 = color;
            current_y += 1;
        } else if current_x > 0
            && prev_position != (current_y, current_x - 1)
            && connects_left(&pipes[current_y][current_x].0)
            && connects_right(&pipes[current_y][current_x - 1].0)
        {
            prev_position = (current_y, current_x);
            pipes[current_y][current_x].1 = color;
            current_x -= 1;
        } else if current_x < pipes[0].len() - 1
            && prev_position != (current_y, current_x + 1)
            && connects_right(&pipes[current_y][current_x].0)
            && connects_left(&pipes[current_y][current_x + 1].0)
        {
            prev_position = (current_y, current_x);
            pipes[current_y][current_x].1 = color;
            current_x += 1;
        } else {
            break None;
        }
    };
}

fn count_inside(pipes: &mut Vec<Vec<(char, usize)>>, color: usize) -> usize {
    let mut count = 0;

    for y in 1..pipes.len() - 1 {
        let mut inside = false;
        for x in 0..pipes[y].len() {
            if pipes[y][x].1 != color {
                if inside {
                    pipes[y][x].0 = 'x';
                    count += 1;
                }
            } else {
                if connects_up(&pipes[y][x].0) && connects_down(&pipes[y - 1][x].0) {
                    inside = !inside;
                }
            }
        }
    }

    return count;
}

pub fn process(input: &str) -> usize {
    let mut pipes = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 0)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_y = 0;
    let mut start_x = 0;
    while pipes[start_y][start_x].0 != 'S' {
        if start_x == pipes[0].len() - 1 {
            start_y += 1;
            start_x = 0;
        } else {
            start_x += 1;
        }
    }

    let mut color = 0;

    // find the loop
    if start_y > 0 && connects_down(&pipes[start_y - 1][start_x].0) {
        if let Some(c) = color_pipe(&mut pipes, (start_y - 1, start_x), (start_y, start_x), 1) {
            color = c;
        }
    }
    // color != 0 means the loop has already been found
    if color == 0 && start_y < pipes.len() - 1 && connects_up(&pipes[start_y + 1][start_x].0) {
        if let Some(c) = color_pipe(&mut pipes, (start_y + 1, start_x), (start_y, start_x), 2) {
            color = c;
        }
    }
    if color == 0 && start_x > 0 && connects_right(&pipes[start_y][start_x - 1].0) {
        if let Some(c) = color_pipe(&mut pipes, (start_y, start_x - 1), (start_y, start_x), 3) {
            color = c;
        }
    }
    if color == 0 && start_x < pipes[0].len() - 1 && connects_left(&pipes[start_y][start_x + 1].0) {
        if let Some(c) = color_pipe(&mut pipes, (start_y, start_x + 1), (start_y, start_x), 4) {
            color = c;
        }
    }
    // loop is already found

    pipes[start_y][start_x].1 = color;

    return count_inside(&mut pipes, color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_case() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(process(input), 10);
    }
}

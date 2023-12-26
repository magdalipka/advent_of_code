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

fn follow_pipe(
    pipes: &Vec<Vec<char>>,
    start_position: (usize, usize),
    prev_position: (usize, usize),
) -> Option<usize> {
    let (mut current_y, mut current_x) = start_position;
    let mut current_length = 1;
    let mut prev_position = prev_position.to_owned();
    return loop {
        if pipes[current_y][current_x] == 'S' {
            break Some(current_length);
        } else if current_y > 0
            && prev_position != (current_y - 1, current_x)
            && connects_up(&pipes[current_y][current_x])
            && connects_down(&pipes[current_y - 1][current_x])
        {
            prev_position = (current_y, current_x);
            current_y -= 1;
        } else if current_y < pipes.len() - 1
            && prev_position != (current_y + 1, current_x)
            && connects_down(&pipes[current_y][current_x])
            && connects_up(&pipes[current_y + 1][current_x])
        {
            prev_position = (current_y, current_x);
            current_y += 1;
        } else if current_x > 0
            && prev_position != (current_y, current_x - 1)
            && connects_left(&pipes[current_y][current_x])
            && connects_right(&pipes[current_y][current_x - 1])
        {
            prev_position = (current_y, current_x);
            current_x -= 1;
        } else if current_x < pipes[0].len() - 1
            && prev_position != (current_y, current_x + 1)
            && connects_right(&pipes[current_y][current_x])
            && connects_left(&pipes[current_y][current_x + 1])
        {
            prev_position = (current_y, current_x);
            current_x += 1;
        } else {
            break None;
        }
        current_length += 1;
    };
}

pub fn process(input: &str) -> usize {
    let pipes = input
        .lines()
        .map(|line| {
            line.chars()
                // .map(|c| (c, None::<usize>))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut start_y = 0;
    let mut start_x = 0;
    while pipes[start_y][start_x] != 'S' {
        if start_x == pipes[0].len() - 1 {
            start_y += 1;
            start_x = 0;
        } else {
            start_x += 1;
        }
    }

    if start_y > 0 {
        if let Some(pipe_length) = follow_pipe(&pipes, (start_y - 1, start_x), (start_y, start_x)) {
            return ((pipe_length as f64) / 2.0).floor() as usize;
        }
    }
    if start_y < pipes.len() - 1 {
        if let Some(pipe_length) = follow_pipe(&pipes, (start_y + 1, start_x), (start_y, start_x)) {
            return ((pipe_length as f64) / 2.0).floor() as usize;
        }
    }
    if start_x > 0 {
        if let Some(pipe_length) = follow_pipe(&pipes, (start_y, start_x - 1), (start_y, start_x)) {
            return ((pipe_length as f64) / 2.0).floor() as usize;
        }
    }
    if start_x < pipes[0].len() - 1 {
        if let Some(pipe_length) = follow_pipe(&pipes, (start_y, start_x + 1), (start_y, start_x)) {
            return ((pipe_length as f64) / 2.0).floor() as usize;
        }
    }

    panic!("Should never be reached!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn first_case() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(process(input), 4);
    }

    #[test]
    fn second_case() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(process(input), 8);
    }
}

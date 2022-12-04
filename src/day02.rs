use std::fs;

fn find_winning_move(a: char) -> char {
    match a {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => a,
    }
}

fn find_losing_move(a: char) -> char {
    match a {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => a,
    }
}

fn find_drawing_move(a: char) -> char {
    match a {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => a,
    }
}

fn calculate_points_for_game(a: char, b: char) -> usize {
    let shape_score = match b {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    let outcome_score = match (a, b) {
        ('C', 'X') | ('A', 'Y') | ('B', 'Z') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        _ => 0,
    };

    shape_score + outcome_score
}

fn play() -> usize {
    let file = fs::read_to_string("./input/day02.txt").unwrap();

    file.split("\n")
        .filter_map(|s| match (s.chars().nth(0), s.chars().nth(2)) {
            (Some(opponent_move), Some(result)) => Some((
                opponent_move,
                match result {
                    'X' => find_losing_move(opponent_move),
                    'Y' => find_drawing_move(opponent_move),
                    'Z' => find_winning_move(opponent_move),
                    _ => result,
                },
            )),
            _ => None,
        })
        .map(|(a, b)| calculate_points_for_game(a, b))
        .fold(0, |xs, x| xs + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02() {
        assert_eq!(play(), 14652);
    }
}

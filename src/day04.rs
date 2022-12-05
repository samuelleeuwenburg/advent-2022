use std::fs;

fn camp_cleanup() -> usize {
    fs::read_to_string("./input/day04.txt")
        .unwrap()
        .split("\n")
        .fold(0, |acc, line| {
            let r: Vec<usize> = line
                .split(&[',', '-'])
                .filter_map(|line| line.parse::<usize>().ok())
                .collect();

            match r.as_slice() {
                &[a1, a2, b1, b2] if (a1 >= b1 && a1 <= b2) || (b1 >= a1 && b1 <= a2) => acc + 1,
                _ => acc,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04() {
        assert_eq!(camp_cleanup(), 870);
    }
}

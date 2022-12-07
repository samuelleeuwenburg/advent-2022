use std::collections::HashSet;
use std::fs;

fn find_marker() -> Option<usize> {
    let mut buffer = vec![];

    for (index, char) in fs::read_to_string("./input/day06.txt")
        .unwrap()
        .chars()
        .enumerate()
    {
        buffer.push(char);
        if buffer.len() > 14 {
            buffer.remove(0);
        }

        let set: HashSet<&char> = HashSet::from_iter(buffer.iter());

        if buffer.len() == 14 && set.len() == 14 {
            return Some(index + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06() {
        assert_eq!(find_marker(), Some(2665));
    }
}

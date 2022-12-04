use std::fs;

fn rugzak() -> usize {
    let (score, _, _) = fs::read_to_string("./input/day03.txt")
        .unwrap()
        .split("\n")
        .fold((0, None, None), |acc, line| match acc {
            (sum, None, None) => (sum, Some(line), None),
            (sum, Some(a), None) => (sum, Some(a), Some(line)),
            (sum, Some(a), Some(b)) => {
                let c = line
                    .chars()
                    .find(|c| a.contains(&c.to_string()) && b.contains(&c.to_string()))
                    .unwrap();

                let offset = if c.is_uppercase() { 38 } else { 96 };

                (sum + c as usize - offset, None, None)
            }
            _ => (0, None, None),
        });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03() {
        assert_eq!(rugzak(), 2567);
    }
}

use std::fs;

fn counting_calories() -> usize {
    let file = fs::read_to_string("./input/day01.txt").unwrap();

    let mut elves: Vec<usize> = vec![0];
    let mut index = 0;

    for line in file.split("\n") {
        if line == "" {
            elves.push(0);
            index += 1;
        } else {
            let calories = line.parse::<usize>().unwrap();
            elves[index] += calories;
        }
    }

    let max1 = *elves.iter().max().unwrap();
    elves.retain(|c| c != &max1);

    let max2 = *elves.iter().max().unwrap();
    elves.retain(|c| c != &max2);

    let max3 = *elves.iter().max().unwrap();
    elves.retain(|c| c != &max3);

    max1 + max2 + max3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1() {
        assert_eq!(counting_calories(), 203420);
    }
}

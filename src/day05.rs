use std::fs;

const COLUMNS: usize = 9;
const ROWS: usize = 8;

fn re_stacks() -> String {
    let file = fs::read_to_string("./input/day05.txt").unwrap();
    let mut init_state: Vec<&str> = file.split("\n").collect();

    let commands: Vec<(usize, usize, usize)> = init_state
        .split_off(ROWS + 2)
        .into_iter()
        .filter(|l| l.len() != 0)
        .map(|line| {
            let values: Vec<usize> = line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .split(" ")
                .filter_map(|line| line.parse::<usize>().ok())
                .collect();

            assert_eq!(values.len(), 3);

            (values[0], values[1], values[2])
        })
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![(); COLUMNS]
        .into_iter()
        .enumerate()
        .map(|(column, _)| {
            let mut stack = vec![];

            for row in (0..ROWS).rev() {
                if let Some(c) = init_state
                    .get(row)
                    .and_then(|l| l.chars().nth(column * 4 + 1))
                    .and_then(|c| if c != ' ' { Some(c) } else { None })
                {
                    stack.push(c);
                }
            }

            stack
        })
        .collect();

    for (amount, from, to) in commands {
        let mut from_stack = stacks.get(from - 1).unwrap().clone();
        let mut to_stack = stacks.get(to - 1).unwrap().clone();

        let mut moved: Vec<char> = from_stack.drain((from_stack.len() - amount)..).collect();

        to_stack.append(&mut moved);

        stacks[from - 1] = from_stack.clone();
        stacks[to - 1] = to_stack.clone();
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05() {
        assert_eq!(&re_stacks(), "DMRDFRHHH");
    }
}

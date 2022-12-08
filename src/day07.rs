use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const TOTAL_SPACE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

fn get_folder_size(paths: &HashMap<String, HashMap<String, usize>>, path: &str) -> usize {
    paths
        .get(path)
        .unwrap_or(&HashMap::new())
        .iter()
        .fold(0, |total, (name, &size)| {
            if size == 0 {
                total + get_folder_size(paths, name)
            } else {
                total + size
            }
        })
}

fn pretty_please_with_sugar_on_top() -> usize {
    let mut paths: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut path = PathBuf::from("/");

    for line in fs::read_to_string("./input/day07.txt").unwrap().split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        let pwd = path.to_str().unwrap().to_string();

        match (parts.get(0), parts.get(1), parts.get(2)) {
            (Some(&"$"), Some(&"cd"), Some(&"..")) => {
                path.pop();
            }
            (Some(&"$"), Some(&"cd"), Some(&"/")) => path = PathBuf::from("/"),
            (Some(&"$"), Some(&"cd"), Some(p)) => path.push(p),
            (Some(size), Some(name), None) => {
                let (size, name) = match size {
                    &"dir" => (0, path.join(name).to_str().unwrap().to_string()),
                    x => (x.parse::<usize>().unwrap_or(0), name.to_string()),
                };

                let set = paths.entry(pwd).or_insert_with(|| HashMap::new());
                set.insert(name, size);
            }
            _ => (),
        }
    }

    let mut folder_sizes: HashMap<String, usize> = HashMap::new();

    for path in paths.keys() {
        folder_sizes.insert(path.into(), get_folder_size(&paths, path));
    }

    let space_required = REQUIRED_SPACE - (TOTAL_SPACE - folder_sizes.get("/").unwrap());

    let mut dirs: Vec<usize> = folder_sizes
        .values()
        .cloned()
        .filter(|&x| x >= space_required)
        .collect();

    dirs.sort();

    *dirs.get(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06() {
        assert_eq!(pretty_please_with_sugar_on_top(), 12545514);
    }
}

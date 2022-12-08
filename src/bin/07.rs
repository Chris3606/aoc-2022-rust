#[derive(Debug)]
struct File {
    _name: String,
    size: u32,
}

impl File {
    pub fn new_from_ref(name: &str, size: u32) -> Self {
        Self {
            _name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<usize>,
}

impl Directory {
    pub fn new_from_ref(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum ItemData {
    File(File),
    Directory(Directory),
}

#[derive(Debug)]
struct Item {
    parent: Option<usize>,
    data: ItemData,
}

#[derive(Debug)]
struct Filesystem {
    items: Vec<Item>,
}

impl Filesystem {
    pub fn new() -> Self {
        Self {
            items: vec![Item {
                parent: None,
                data: ItemData::Directory(Directory::new_from_ref("/")),
            }],
        }
    }

    pub fn get_child_directory_idx(&self, cur_dir: &Directory, name: &str) -> Option<usize> {
        for &child in &cur_dir.children {
            if let ItemData::Directory(d) = &self.items[child].data {
                if d.name == name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn get_dir_from_idx(&self, idx: usize) -> Option<&Directory> {
        if idx >= self.items.len() {
            return None;
        }

        let item = &self.items[idx];
        return if let ItemData::Directory(d) = &item.data {
            Some(d)
        } else {
            None
        };
    }

    pub fn get_dir_from_idx_mut(&mut self, idx: usize) -> Option<&mut Directory> {
        if idx >= self.items.len() {
            return None;
        }

        let item = &mut self.items[idx];
        return if let ItemData::Directory(d) = &mut item.data {
            Some(d)
        } else {
            None
        };
    }
}

fn filesystem_from_cmd_history(history: &str) -> Filesystem {
    let mut filesystem = Filesystem::new();
    let mut cur_dir: usize = 0;

    for line in history.lines().skip(1) {
        if line.starts_with('$') {
            let mut cmd_it = line[2..].split(' ');
            let cmd = cmd_it.next().unwrap();

            match cmd {
                "cd" => {
                    let arg = cmd_it.next().unwrap();

                    // If this is the first time we're seeing the directory, add it
                    cur_dir = match arg {
                        ".." => filesystem.items[cur_dir].parent.unwrap(),
                        s => {
                            let dir_idx = filesystem
                                .get_child_directory_idx(
                                    filesystem.get_dir_from_idx(cur_dir).unwrap(),
                                    s,
                                )
                                .unwrap();

                            dir_idx
                        }
                    };
                }
                "ls" => {} // Doesn't matter, we only need the current directory
                _ => panic!("Invalid input."),
            }
        } else {
            let mut listing_it = line.split(' ');
            let parts = (listing_it.next().unwrap(), listing_it.next().unwrap());

            if parts.0 == "dir" {
                // Add directory to the current directory
                let dir = Directory::new_from_ref(parts.1);
                filesystem.items.push(Item {
                    parent: Some(cur_dir),
                    data: ItemData::Directory(dir),
                });
                let idx = filesystem.items.len() - 1;

                filesystem
                    .get_dir_from_idx_mut(cur_dir)
                    .unwrap()
                    .children
                    .push(idx);
            } else {
                // Add file to the current directory
                let file = File::new_from_ref(parts.1, parts.0.parse::<u32>().unwrap());
                filesystem.items.push(Item {
                    parent: Some(cur_dir),
                    data: ItemData::File(file),
                });

                let idx = filesystem.items.len() - 1;
                filesystem
                    .get_dir_from_idx_mut(cur_dir)
                    .unwrap()
                    .children
                    .push(idx);
            }
        }
    }

    filesystem
}

fn get_dir_size_recursive(fs: &Filesystem, dir: &Directory, sizes: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    for &child_idx in &dir.children {
        let child = &fs.items[child_idx];
        sum += match &child.data {
            ItemData::Directory(d) => get_dir_size_recursive(fs, d, sizes),
            ItemData::File(f) => f.size,
        };
    }

    sizes.push(sum);
    sum
}

fn get_dir_sizes(fs: &Filesystem) -> Vec<u32> {
    let mut result = Vec::new();
    let root = fs.get_dir_from_idx(0).unwrap();

    get_dir_size_recursive(fs, root, &mut result);

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let filesystem = filesystem_from_cmd_history(input);

    let dir_sizes = get_dir_sizes(&filesystem);
    let sum_sizes = dir_sizes.iter().filter(|&i| *i <= 100000).sum();

    Some(sum_sizes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let filesystem = filesystem_from_cmd_history(input);

    const TOTAL_DISK_SPACE: u32 = 70000000;
    const UNUSED_SPACE_NEEDED: u32 = 30000000;

    let dir_sizes = get_dir_sizes(&filesystem);
    let free_space = TOTAL_DISK_SPACE - dir_sizes[dir_sizes.len() - 1];
    let needed_space = UNUSED_SPACE_NEEDED - free_space;

    let size_of_target_dir = dir_sizes
        .iter()
        .filter(|&i| *i >= needed_space)
        .min()
        .unwrap();

    Some(*size_of_target_dir)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

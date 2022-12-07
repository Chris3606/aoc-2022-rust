#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn new_from_ref(name: &str, size: u32) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
}

impl Directory {
    pub fn new_from_ref(name: &str) -> Self {
        Self {
            name: name.to_string(),
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
        Self { items: Vec::new() }
    }

    // pub fn get_directory(&self, name: &str) -> Option<&Directory> {
    //     for i in &self.items {
    //         if let Item::Directory(d) = i {
    //             if d.name == name {
    //                 return Some(d);
    //             }
    //         }
    //     }
    //     None
    // }

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
}

fn filesystem_from_cmd_history(history: &str) {}

pub fn part_one(input: &str) -> Option<u32> {
    let mut filesystem = Filesystem::new();
    let mut cur_dir: Option<usize> = None;

    for line in input.lines() {
        if line.starts_with('$') {
            let mut cmd_it = line[2..].split(' ');
            let cmd = cmd_it.next().unwrap();

            match cmd {
                "cd" => {
                    let arg = cmd_it.next().unwrap();

                    // If this is the first time we're seeing the directory, add it
                    cur_dir = match arg {
                        ".." => filesystem.items[cur_dir.unwrap()].parent,
                        s => {
                            let dir = Directory::new_from_ref(s);
                            filesystem.items.push(Item {
                                parent: cur_dir,
                                data: ItemData::Directory(dir),
                            });
                            Some(filesystem.items.len() - 1)
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
                    parent: cur_dir,
                    data: ItemData::Directory(dir),
                });
            } else {
                // Add file to the current directory
                let file = File::new_from_ref(parts.1, parts.0.parse::<u32>().unwrap());
                filesystem.items.push(Item {
                    parent: cur_dir,
                    data: ItemData::File(file),
                });
            }
        }
    }
    dbg!(filesystem);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}

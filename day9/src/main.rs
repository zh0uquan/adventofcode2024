use itertools::Itertools;
use sorted_vec::SortedVec;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[derive(Debug, Clone)]
struct File {
    address: usize,
    size: usize,
    id: usize,
}

#[derive(Debug, Clone, Eq)]
struct Space {
    address: usize,
    size: usize,
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> Ordering {
        other.address.cmp(&self.address)
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.size == other.size
    }
}

fn get_disk_map(input: &str) -> (Vec<String>, Vec<File>, SortedVec<Space>) {
    let mut address = 0;
    input.trim().bytes().chunks(2).into_iter().enumerate().fold(
        (vec![], vec![], SortedVec::new()),
        |(mut s, mut files, mut spaces), (i, chunk)| {
            let chunk: Vec<u8> = chunk.map(|b| b - b'0').collect();
            match chunk.len() {
                1 => {
                    let file_size = chunk[0] as usize;
                    s.extend(vec![i.to_string(); file_size]);
                    files.push(File {
                        address,
                        size: file_size,
                        id: i,
                    });
                    address += file_size;
                }
                2 => {
                    let (file_size, space_size) =
                        (chunk[0] as usize, chunk[1] as usize);
                    s.extend(vec![i.to_string(); file_size]);
                    files.push(File {
                        address,
                        size: file_size,
                        id: i,
                    });
                    address += file_size;
                    s.extend(vec![".".to_string(); space_size]);
                    spaces.push(Space {
                        address,
                        size: space_size,
                    });
                    address += space_size;
                }
                _ => panic!("disco!"),
            }
            (s, files, spaces)
        },
    )
}

fn part1(input: &str) -> usize {
    let mut disk_map = get_disk_map(input).0;
    let (mut i, mut j) = (0, disk_map.len() - 1);
    while i < j {
        if disk_map[i] != "." {
            i += 1;
            continue;
        }
        while disk_map[j] == "." {
            j -= 1;
        }
        disk_map.swap(i, j);
    }
    disk_map
        .iter()
        .filter(|&n| *n != ".")
        .enumerate()
        .map(|(i, n)| i * n.parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    let (mut _map, mut files, mut spaces) = get_disk_map(input);

    while !spaces.is_empty() {
        let mut space = spaces.pop().unwrap();
        for file in files.iter_mut().rev() {
            if file.size <= space.size && file.address > space.address {
                spaces.push(Space {
                    address: file.address,
                    size: file.size,
                });
                file.address = space.address;
                space.address += file.size;
                space.size -= file.size;
            }
        }
    }
    files
        .iter()
        .map(|file| {
            (file.address..file.address + file.size)
                .map(|n| n * file.id)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            get_disk_map("12345").0.join(""),
            "0..111....22222".to_string()
        );
        assert_eq!(
            get_disk_map("2333133121414131402").0.join(""),
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        );
        assert_eq!(part1("2333133121414131402"), 1928)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2333133121414131402"), 2858)
    }
}

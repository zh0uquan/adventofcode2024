use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn get_disk_map(input: &str) -> Vec<String> {
    input.trim().bytes().chunks(2).into_iter().enumerate().fold(
        vec![], |mut s, (i, chunk)| {
            let chunk: Vec<u8> = chunk.map(|b| b - b'0').collect();
            match chunk.len() {
                1 => {
                    let file_size = chunk[0] as usize;
                    s.extend(vec![ i.to_string(); file_size]);
                },
                2 => {
                    let (file_size, space_size) = (chunk[0], chunk[1]);
                    s.extend(vec![ i.to_string(); file_size as usize]);
                    s.extend(vec![ ".".to_string(); space_size as usize]);
                }
                _ => panic!("disco!")
            }
            s
        }
    )
}



fn part1(input: &str) -> usize {

    let mut disk_map = get_disk_map(input);
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
    disk_map.iter()
        .filter(|&n| *n != ".")
        .enumerate()
        .map(|(i, n)| i * n.parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {

        assert_eq!(get_disk_map("12345").join(""), "0..111....22222".to_string());
        assert_eq!(get_disk_map("2333133121414131402").join(""), "00...111...2...333.44.5555.6666.777.888899".to_string());
        assert_eq!(part1("2333133121414131402"), 1928)
    }


    #[test]
    fn test_part2() {
        // assert_eq!();
    }
}

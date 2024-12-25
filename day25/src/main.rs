use common::Matrix;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

trait Height {
    fn get_columns(&self) -> (&str, Vec<usize>);
}

impl Height for Matrix<char> {
    fn get_columns(&self) -> (&str, Vec<usize>) {
        let mut counts = vec![];
        for j in 0..self.width {
            let mut count = 0;
            for i in 0..self.height {
                if self.matrix[i][j] == '#' {
                    count += 1;
                }
            }
            counts.push(count);
        }
        if (0..self.width)
            .filter(|&j| self.matrix[0][j] == '#')
            .count()
            == self.width
        {
            return ("lock", counts.iter().map(|n| n - 1).collect());
        }
        ("key", counts.iter().map(|n| n - 1).collect())
    }
}

fn part1(input: &str) {
    let matrixes: Vec<Matrix<char>> = input
        .split("\n\n")
        .map(|block| Matrix::from(block, |c| c))
        .collect();

    let overlap_height = matrixes[0].height - 2;
    let width = matrixes[0].width;

    let (keys, locks) = matrixes.iter().map(|m| m.get_columns()).fold(
        (vec![], vec![]),
        |(mut keys, mut locks), (t, columns)| {
            match t {
                "key" => keys.push(columns),
                "lock" => locks.push(columns),
                _ => panic!("disco!"),
            }
            (keys, locks)
        },
    );

    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            println!("{:?} {:?}", lock, key);
            if (0..width)
                .filter(|&i| key[i] + lock[i] <= overlap_height)
                .count()
                == width
            {
                count += 1;
            }
        }
    }
    println!("{count}")
}

fn part2(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....

            #####
            ##.##
            .#.##
            ...##
            ...#.
            ...#.
            .....

            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####

            .....
            .....
            #.#..
            ###..
            ###.#
            ###.#
            #####

            .....
            .....
            .....
            #....
            #.#..
            #.#.#
            #####
            "#
        };
        part1(input)
        // assert_eq!();
    }

    #[test]
    fn test_part2() {
        // assert_eq!();
    }
}

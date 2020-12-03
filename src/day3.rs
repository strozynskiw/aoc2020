type Map = Vec<Vec<bool>>;

fn parse(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Incorrect char"),
        })
        .collect()
}

fn sum_trees(map: &[Vec<bool>], down: usize, right: usize) -> usize {
    map.iter()
        .step_by(down)
        .enumerate()
        .map(|(i, r)| *r.iter().cycle().nth(i * right).unwrap() as usize)
        .sum()
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| parse(l))
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part_01(input: &[Vec<bool>]) -> usize {
    sum_trees(&input, 1, 3)
}

#[aoc(day3, part2)]
pub fn solve_part_02(input: &[Vec<bool>]) -> usize {
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes
        .iter()
        .fold(1, |acc, s| acc * sum_trees(&input, s.0, s.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

        assert_eq!(solve_part_01(&input_generator(data)), 7)
    }

    #[test]
    fn sample_02() {
        let data = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

        assert_eq!(solve_part_02(&input_generator(data)), 336)
    }
}

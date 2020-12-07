use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_owned()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part_01(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| s.replace("\n", "").chars().collect::<HashSet<_>>().len())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part_02(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| {
            let answers: Vec<String> = s
                .split("\n")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s|s.to_owned())
                .collect();

            let letters: Vec<HashSet<char>> = answers
                .iter()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect();

            let mut result: HashSet<char> = letters.get(0).unwrap().clone();

            for set in letters {
                result = set.intersection(&result).copied().collect();
            }

            result.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
abc

a
b
c

ab
ac

a
a
a
a

b
";

        assert_eq!(solve_part_01(&input_generator(data)), 11)
    }

    #[test]
    fn sample_02() {
        let data = "
abc

a
b
c

ab
ac

a
a
a
a

b
";

        assert_eq!(solve_part_02(&input_generator(data)), 6)
    }
}

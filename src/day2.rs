use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // "2-4 p: vpkpp"
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

pub struct Row {
    password: String,
    character: char,
    upper_bound: usize,
    lower_bound: usize,
}

fn parse(line: &str) -> Option<Row> {
    let results = RE.captures(line)?;
    let lower_bound = results[1].parse::<usize>().expect("Parsing error");
    let upper_bound = results[2].parse::<usize>().expect("Parsing error");

    Some(Row {
        password: results[4].parse::<String>().expect("Parsing error"),
        character: results[3].parse::<char>().expect("Parsing error"),
        upper_bound,
        lower_bound,
    })
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|l| parse(l))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part_01(input: &Vec<Row>) -> usize {
    input
        .iter()
        .filter(|r| {
            (r.lower_bound..=r.upper_bound)
                .contains(&r.password.chars().filter(|&c| c == r.character).count())
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part_02(input: &Vec<Row>) -> usize {
    input
        .iter()
        .filter(|r| {
            let first = r.password.chars().nth(r.lower_bound - 1).unwrap();
            let second = r.password.chars().nth(r.upper_bound - 1).unwrap();

            first != second && (first == r.character || second == r.character)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
";

        assert_eq!(solve_part_01(&input_generator(data)), 2)
    }

    #[test]
    fn sample_02() {
        let data = "
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
";

        assert_eq!(solve_part_02(&input_generator(data)), 1)
    }
}

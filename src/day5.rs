use std::ops::RangeInclusive;

fn half(range: &RangeInclusive<u32>) -> u32 {
    ((*range.end() as f32 - *range.start() as f32) / 2 as f32).round() as u32
}

fn find_seat_id(ticket: &str) -> u32 {
    let mut row = 0..=127;
    let mut col = 0..=7;

    ticket.chars().for_each(|c| match c {
        'L' => col = RangeInclusive::new(*col.start(), *col.end() - half(&col)),
        'R' => col = RangeInclusive::new(*col.start() + half(&col), *col.end()),
        'F' => row = RangeInclusive::new(*row.start(), *row.end() - half(&row)),
        'B' => row = RangeInclusive::new(*row.start() + half(&row), *row.end()),
        _ => panic!("Incorrect code!"),
    });

    row.end() * 8 + col.end()
}

#[test]
fn test_seat_id() {
    assert_eq!(find_seat_id("FBFBBFFRLR"), 357)
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| String::from(l))
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part_01(input: &[String]) -> u32 {
    input
        .iter()
        .map(|ticket| find_seat_id(ticket))
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn solve_part_02(input: &[String]) -> u32 {
    let mut ids: Vec<u32> = input.iter().map(|code| find_seat_id(code)).collect();
    ids.sort();
    println!("{:#?}", ids);

    // just find first missing seat
    for i in 0..ids.len() {
        if ids[i] + 1 != ids[i + 1] {
            return ids[i] + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL
";

        assert_eq!(solve_part_01(&input_generator(data)), 820)
    }

    #[test]
    fn sample_02() {
        let data = "

";

        assert_eq!(solve_part_02(&input_generator(data)), 336)
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

// naive solution just for now
#[aoc(day1, part1)]
pub fn solve_part_01(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    0
}

// naive solution just for now
#[aoc(day1, part2)]
pub fn solve_part_02(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
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
1721
979
366
299
675
1456
";

        assert_eq!(solve_part_01(&input_generator(data)), 514579)
    }

    #[test]
    fn sample_02() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(solve_part_02(&input_generator(data)), 241861950)
    }
}

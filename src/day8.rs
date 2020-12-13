use std::collections::HashSet;
struct Machine {
    acc: i32,
}

impl Machine {
    pub fn new() -> Machine {
        Machine { acc: 0 }
    }

    fn parse(&mut self, instruction: &str) -> i32 {
        let mut parts = instruction.split(" ");
        let ins = parts.next().expect("no instruction");
        let args: Vec<&str> = parts.collect();

        match ins {
            "nop" => 1,
            "jmp" => args
                .get(0)
                .expect("Incorrect argument")
                .parse::<i32>()
                .expect("Parsing error"),
            "acc" => {
                self.acc += args
                    .get(0)
                    .expect("Incorrect argument")
                    .parse::<i32>()
                    .expect("Parsing error");
                1
            }
            _ => panic!("Incorrect instruction"),
        }
    }

    pub fn exec(&mut self, instructions: &[String]) -> bool {
        let mut ic = 0;
        let mut seen: HashSet<usize> = HashSet::new();
        while ic < instructions.len() {
            if seen.contains(&ic) {
                return false;
            }
            seen.insert(ic);
            ic = (ic as i32 + Machine::parse(self, instructions.get(ic).unwrap())) as usize;
        }
        true
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part_01(input: &[String]) -> i32 {
    let mut mach = Machine::new();
    mach.exec(input);
    mach.acc
}

#[aoc(day8, part2)]
pub fn solve_part_02(input: &[String]) -> i32 {
    for i in 0..input.len() {
        let instructions: Vec<String> = input
            .iter()
            .enumerate()
            .map(|(j, l)| {
                if i == j {
                    if l.contains("jmp") {
                        return l.replace("jmp", "nop").to_owned();
                    } else {
                        return l.replace("nop", "jmp").to_owned();
                    }
                } else {
                    return l.to_owned();
                }
            })
            .collect();

        let mut mach = Machine::new();

        match mach.exec(&instructions) {
            true => return mach.acc,
            false => continue,
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
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

        assert_eq!(solve_part_01(&input_generator(data)), 5)
    }

    #[test]
    fn sample_02() {
        let data = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

        assert_eq!(solve_part_02(&input_generator(data)), 8)
    }
}

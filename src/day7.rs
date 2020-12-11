use std::collections::HashMap;
type BagContent = Vec<(String, u32)>;
type Rules = HashMap<String, BagContent>;

fn parse(line: &str) -> (String, BagContent) {
    //light red bags contain 1 bright white bag, 2 muted yellow bags.
    //bright white bags contain 1 shiny gold bag.
    //faded blue bags contain no other bags.

    let mut parts = line.split(" bags contain ");

    let core = parts.next().expect("No core :(");
    let bags = parts.next().expect("No bags :(");

    match bags {
        "no other bags." => (core.to_owned(), Vec::new()),
        _ => {
            let content_strings: Vec<String> = bags
                .replace(".", "")
                .split(",")
                .map(|s| s.to_owned())
                .collect();
            let mut content = BagContent::new();
            content_strings.iter().map(|c| c.trim()).for_each(|c| {
                let mut content_parts = c.split(" ");
                let number = content_parts
                    .next()
                    .expect("WUT?")
                    .parse::<u32>()
                    .expect("No way!");
                let name = format!(
                    "{} {}",
                    content_parts.next().expect("WUT?"),
                    content_parts.next().expect("WUT?")
                );
                content.push((name.clone(), number));
            });
            (core.to_owned(), content)
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Rules {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| parse(l))
        .collect()
}

fn contains_bag(bag_name: &str, rules: &Rules, content: &BagContent) -> bool {
    if content.iter().any(|r| r.0 == bag_name) {
        return true;
    } else {
        return content.iter().fold(false, |acc, r| {
            acc | contains_bag(bag_name, rules, rules.get(&r.0).expect("no such bag"))
        });
    }
}

fn count_bags(rules: &Rules, color: &str) -> u32 {
    let mut total = 0;
    if let Some(content) = rules.get(color) {
        for (color, content) in content {
            total += content;
            total += content * count_bags(rules, color);
        }
    }
    total
}

#[aoc(day7, part1)]
pub fn solve_part_01(input: &Rules) -> usize {
    // -1 to avoid counting the shiny gold as containing shiny gold bag.
    input
        .iter()
        .filter(|r| contains_bag("shiny gold", input, &r.1))
        .count()
}

#[aoc(day7, part2)]
pub fn solve_part_02(input: &Rules) -> u32 {
    count_bags(input, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

        assert_eq!(solve_part_01(&input_generator(data)), 4)
    }

    #[test]
    fn sample_02() {
        let data = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

        assert_eq!(solve_part_02(&input_generator(data)), 126)
    }
}

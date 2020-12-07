use std::collections::HashMap;
type PassportData = HashMap<String, String>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    let raw_data: Vec<String> = input
        .split("\n\n")
        .map(|s| String::from(s).replace("\n", ""))
        .collect();

    raw_data
        .iter()
        .map(|rd| {
            let data: Vec<String> = rd.split(" ").map(|s| String::from(s)).collect();

            let mut pd: PassportData = PassportData::new();

            data.iter()
                .map(|d| d.trim())
                .filter(|d| !d.is_empty())
                .for_each(|d| {
                    let values: Vec<String> = d.split(":").map(|s| String::from(s)).collect();
                    assert!(values.len() == 2);
                    pd.insert(values[0].clone(), values[1].clone());
                });
            pd
        })
        .collect()
}

pub fn validate_passport(data: &PassportData) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&d| data.contains_key(d))
}

#[aoc(day4, part1)]
pub fn solve_part_01(input: &[PassportData]) -> usize {
    input.iter().filter(|i| validate_passport(i)).count()
}

pub fn validate_passport_data(data: &PassportData) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&d| data.contains_key(d))
}

trait Valid {
    fn validate(data: &str) -> bool;
}

struct ByrValidator;
struct IyrValidator;
struct EyrValidator;
struct HgtValidator;
struct HclValidator;
struct EclValidator;
struct PidValidator;

impl Valid for ByrValidator {
    fn validate(data: &str) -> bool {
        let result = data.parse::<i32>();
        match result {
            Ok(x) => (1920..=2002).contains(&x),
            Err(_) => false,
        }
    }
}

impl Valid for IyrValidator {
    fn validate(data: &str) -> bool {
        let result = data.parse::<i32>();
        match result {
            Ok(x) => (2010..=2020).contains(&x),
            Err(_) => false,
        }
    }
}

impl Valid for EyrValidator {
    fn validate(data: &str) -> bool {
        let result = data.parse::<i32>();
        data.len() == 4
            && match result {
                Ok(x) => (2020..=2030).contains(&x),
                Err(_) => false,
            }
    }
}

impl Valid for HgtValidator {
    fn validate(data: &str) -> bool {
        if data.len() < 2 {
            return false;
        }
        let unit: String = data.chars().skip(data.len() - 2).collect();
        let value: String = data.chars().take(data.len() - 2).collect();
        let value = value.parse::<i32>().unwrap_or(0);
        match unit.as_str() {
            "cm" => (150..=193).contains(&value),
            "in" => (59..=76).contains(&value),
            _ => false,
        }
    }
}

impl Valid for HclValidator {
    fn validate(data: &str) -> bool {
        let correct_first_char = data.starts_with('#');
        let value: String = data.chars().skip(1).collect();

        correct_first_char
            && value
                .chars()
                .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
    }
}

impl Valid for EclValidator {
    fn validate(data: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|&c| c == data)
    }
}

impl Valid for PidValidator {
    fn validate(data: &str) -> bool {
        data.len() == 9 && data.chars().all(|c| ('0'..='9').contains(&c))
    }
}

#[aoc(day4, part2)]
pub fn solve_part_02(input: &[PassportData]) -> usize {
    input
        .iter()
        .filter(|pd| {
            ByrValidator::validate(pd.get("byr").unwrap_or(&String::new()))
                && IyrValidator::validate(pd.get("iyr").unwrap_or(&String::new()))
                && EyrValidator::validate(pd.get("eyr").unwrap_or(&String::new()))
                && HgtValidator::validate(pd.get("hgt").unwrap_or(&String::new()))
                && HclValidator::validate(pd.get("hcl").unwrap_or(&String::new()))
                && EclValidator::validate(pd.get("ecl").unwrap_or(&String::new()))
                && PidValidator::validate(pd.get("pid").unwrap_or(&String::new()))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

";

        assert_eq!(solve_part_01(&input_generator(data)), 2)
    }

    #[test]
    fn sample_02() {
        let data = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

";

        assert_eq!(solve_part_02(&input_generator(data)), 4)
    }

    #[test]
    fn byr_test() {
        assert_eq!(ByrValidator::validate("2002"), true);
        assert_eq!(ByrValidator::validate("2003"), false);
        assert_eq!(ByrValidator::validate("asdasd"), false);
        assert_eq!(ByrValidator::validate(""), false);
    }

    #[test]
    fn iyr_test() {
        assert_eq!(IyrValidator::validate("2010"), true);
        assert_eq!(IyrValidator::validate("2020"), true);
        assert_eq!(IyrValidator::validate("2021"), false);
        assert_eq!(IyrValidator::validate("2009"), false);
        assert_eq!(IyrValidator::validate("asdasd"), false);
        assert_eq!(IyrValidator::validate(""), false);
    }

    #[test]
    fn eyr_test() {
        assert_eq!(EyrValidator::validate("2020"), true);
        assert_eq!(EyrValidator::validate("2030"), true);
        assert_eq!(EyrValidator::validate("20031"), false);
        assert_eq!(EyrValidator::validate("2031"), false);
    }

    #[test]
    fn hgt_test() {
        assert_eq!(HgtValidator::validate("60in"), true);
        assert_eq!(HgtValidator::validate("190cm"), true);
        assert_eq!(HgtValidator::validate("190in"), false);
        assert_eq!(HgtValidator::validate("190"), false);
    }

    #[test]
    fn hcl_test() {
        assert_eq!(HclValidator::validate("#123abc"), true);
        assert_eq!(HclValidator::validate("#111111"), true);
        assert_eq!(HclValidator::validate("123abc"), false);
        assert_eq!(HclValidator::validate("#cvgdx"), false);
        assert_eq!(HclValidator::validate(""), false);
    }

    #[test]
    fn ecl_test() {
        assert_eq!(EclValidator::validate("grn"), true);
        assert_eq!(EclValidator::validate("blu"), true);
        assert_eq!(EclValidator::validate("gty"), false);
        assert_eq!(EclValidator::validate("asdasd"), false);
    }

    #[test]
    fn pid_test() {
        assert_eq!(PidValidator::validate("000000001"), true);
        assert_eq!(PidValidator::validate("1000000001"), false);
        assert_eq!(PidValidator::validate("0000001"), false);
        assert_eq!(PidValidator::validate("lsllsllsl"), false);
    }
}

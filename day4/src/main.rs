use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = input::read_paragraphs("inputs/4")?;

    let count = input.iter().filter(|e| is_valid_pt1(*e)).count();
    println!("part 1: {}", count);

    let count = input.iter().filter(|e| is_valid_pt2(*e)).count();
    println!("part 2: {}", count);

    Ok(())
}

fn is_valid_pt1(passport: &str) -> bool {
    passport.contains("byr")
        && passport.contains("iyr")
        && passport.contains("eyr")
        && passport.contains("hgt")
        && passport.contains("hcl")
        && passport.contains("ecl")
        && passport.contains("pid")
}

fn is_valid_pt2(passport: &str) -> bool {
    // byr, iyr, eyr, hgt, hcl, ecl, pid, cid
    let mut valid: u8 = 0;
    let entries = passport.split(|c: char| c.is_whitespace() || c == '\n').filter(|s| s.trim() != "");
    for entry in entries {
        match &entry[0..3] {
            "byr" => match entry[4..].parse() {
                Ok(1920..=2002) => valid |= 1 << 7,
                _ => continue,
            },
            "iyr" => match entry[4..].parse() {
                Ok(2010..=2020) => valid |= 1 << 6,
                _ => continue,
            },
            "eyr" => match entry[4..].parse() {
                Ok(2020..=2030) => valid |= 1 << 5,
                _ => continue,
            },
            "hgt" => {
                let rest = &entry[4..];
                let unit = rest.trim_start_matches(|c: char| c.is_numeric());
                let num = rest.strip_suffix(unit).unwrap().parse();
                match (unit, num) {
                    ("cm", Ok(150..=193)) => valid |= 1 << 4,
                    ("in", Ok(59..=76)) => valid |= 1 << 4,
                    _ => continue,
                }
            }
            "hcl" => {
                if entry.chars().nth(4) != Some('#') {
                    continue;
                }
                // Discard every correct char, check if there are any left
                if entry[5..]
                    .chars()
                    .filter(|c| !match c {
                        '0'..='9' => true,
                        'a'..='f' => true,
                        _ => false,
                    })
                    .count()
                    == 0
                {
                    valid |= 1 << 3
                }
            }
            "ecl" => match &entry[4..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => valid |= 1 << 2,
                _ => continue,
            },
            "pid" => {
                if entry[4..].chars().filter(|c| c.is_numeric()).count() == 9 {
                    valid |= 1 << 1;
                }
            }
            // Doesn't matter, could be removed
            "cid" => valid |= 1,
            _ => unreachable!(),
        }
    }
    // Last bit is optional
    valid >= u8::MAX - 1
}

#[test]
fn test_part_1() {
    let input = "
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
    let input: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(input.iter().filter(|e| is_valid_pt1(*e)).count(), 2);
}

#[test]
fn test_valid() {
    let input = "
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
    let input: Vec<&str> = input.split("\n\n").collect();
    for pass in input {
        assert!(is_valid_pt2(pass))
    }
}

/*
#[test]
fn test_invalid() {
    let input = "
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
    ";
    let input: Vec<&str> = input.split("\n\n").collect();
    for pass in input {
        assert!(!is_valid_pt2(pass))
    }
}
*/
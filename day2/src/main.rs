use std::{ops::RangeInclusive, str::FromStr};

use anyhow::Result;
use input::read_lines;

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    times: RangeInclusive<u16>,
    letter: char,
    password: String,
}

impl FromStr for Entry {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hyphen = s.find('-').unwrap();
        let space = s.find(' ').unwrap();
        let colon = s.find(':').unwrap();

        let range_start = (&s[..hyphen]).parse::<u16>()?;
        let range_end = (&s[hyphen + 1..space]).parse::<u16>()?;

        let letter = s.chars().nth(space + 1).unwrap();

        let password = s[colon + 2..].to_string();

        let entry = Entry {
            times: range_start..=range_end,
            letter,
            password,
        };
        Ok(entry)
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = read_lines("inputs/2")?;

    let entries: Vec<Entry> = input
        .into_iter()
        .map(|s| Entry::from_str(&s).unwrap())
        .collect();

    // Part One
    println!(
        "part 1: {}",
        entries.iter().filter(|e| is_valid_pt1(e)).count()
    );

    // Part Two
    println!(
        "part 2: {}",
        entries.iter().filter(|e| is_valid_pt2(e)).count()
    );

    Ok(())
}

fn is_valid_pt1(pass: &Entry) -> bool {
    let num_letters = pass.password.chars().filter(|c| *c == pass.letter).count() as u16;

    pass.times.contains(&num_letters)
}

fn is_valid_pt2(pass: &Entry) -> bool {
    let left = pass
        .password
        .chars()
        .nth(*pass.times.start() as usize - 1)
        .unwrap()
        == pass.letter;
    let right = pass
        .password
        .chars()
        .nth(*pass.times.end() as usize - 1)
        .unwrap()
        == pass.letter;

    left ^ right
}

#[test]
fn test_entry_from_str() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let mut c = input.into_iter().map(|s| Entry::from_str(s).unwrap());

    assert_eq!(
        c.next(),
        Some(Entry {
            times: 1..=3,
            letter: 'a',
            password: "abcde".to_string(),
        })
    );
    assert_eq!(
        c.next(),
        Some(Entry {
            times: 1..=3,
            letter: 'b',
            password: "cdefg".to_string(),
        })
    );
    assert_eq!(
        c.next(),
        Some(Entry {
            times: 2..=9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        })
    );
    assert_eq!(c.next(), None);
}

#[test]
fn test_is_valid_pt1() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let mut c = input.into_iter().map(|s| Entry::from_str(s).unwrap());
    assert!(is_valid_pt1(&c.next().unwrap()));
    assert!(!is_valid_pt1(&c.next().unwrap()));
    assert!(is_valid_pt1(&c.next().unwrap()));
}

#[test]
fn test_is_valid_pt2() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let mut c = input.into_iter().map(|s| Entry::from_str(s).unwrap());
    assert!(is_valid_pt2(&c.next().unwrap()));
    assert!(!is_valid_pt2(&c.next().unwrap()));
    assert!(!is_valid_pt2(&c.next().unwrap()));
}

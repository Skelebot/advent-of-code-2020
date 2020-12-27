use std::ops::Deref;

use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = input::read_paragraphs("inputs/6")?;
    println!("part 1: {}", count_answered_any(&input));
    println!("part 2: {}", count_answered_every(&input));
    Ok(())
}

fn count_answered_any<T: Deref<Target = str>>(groups: &[T]) -> usize {
    groups
        .iter()
        .map(|s| ('a'..='z').filter(|c| s.contains(*c)).count())
        .sum()
}

fn count_answered_every<T: Deref<Target = str> + std::fmt::Debug>(groups: &[T]) -> usize {
    let mut sum = 0;
    for group in groups.iter() {
        let mut count = 0;
        let first_line = group.lines().next().unwrap();

        for check_c in first_line.chars() {
            let mut every = true;
            for line in group.lines().skip(1) {
                if !line.contains(check_c) {
                    every = false;
                    break;
                }
            }
            if every {
                count += 1
            }
        }
        sum += count
    }
    sum
}

#[test]
fn test_count_answered_any() {
    let input = "
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

    let groups: Vec<&str> = input.split("\n\n").collect();
    let count = count_answered_any(&groups);
    assert_eq!(count, 11);
}

#[test]
fn test_count_answered_every() {
    let input = "
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

    let groups: Vec<&str> = input.split("\n\n").map(|g| g.trim()).collect();
    println!("groups: {:#?}", groups);
    let count = count_answered_every(&groups);
    assert_eq!(count, 6);
}

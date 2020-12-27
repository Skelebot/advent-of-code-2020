use std::ops::Deref;

use anyhow::Result;

pub const MAX_CONTAINED_BAGS: usize = 8;

fn main() -> Result<()> {
    let input: Vec<String> = input::read_lines("inputs/7")?;
    let bag_names = get_bag_names(&input);
    let bags = parse_bag_rules(&input, &bag_names);

    let bag_id = bag_names.iter().position(|c| c == "shiny gold").unwrap();

    let count = bags.iter().filter(|b| b.can_contain(bag_id, &bags)).count();
    println!("part 1: {}", count);

    // count_contained adds the containing (top-level) bag too
    let count = bags.get(bag_id).unwrap().count_contained(&bags) - 1;
    println!("part 2: {}", count);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContainmentRule {
    pub count: u8,
    pub id: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Bag {
    pub id: usize,
    pub contains: [Option<ContainmentRule>; 8],
}

impl Bag {
    pub fn can_contain(&self, bag_id: usize, bags: &[Bag]) -> bool {
        if self
            .contains
            .iter()
            .any(|c| c.is_some() && c.unwrap().id == bag_id)
        {
            true
        } else {
            for rule in self.contains.iter() {
                if let Some(ContainmentRule {
                    id: contained_id, ..
                }) = rule
                {
                    if bags[*contained_id].can_contain(bag_id, bags) {
                        return true;
                    }
                }
            }
            false
        }
    }
    pub fn count_contained(&self, bags: &[Bag]) -> usize {
        let mut sum = 1;
        for rule in self.contains.iter() {
            if let Some(ContainmentRule { id, count }) = rule {
                sum += *count as usize * bags[*id].count_contained(bags)
            }
        }
        sum
    }
}

fn get_bag_names<T: Deref<Target = str>>(input: &[T]) -> Vec<String> {
    let mut names = Vec::new();
    for line in input {
        // Take the first two words
        let second_space = line.match_indices(' ').nth(1).unwrap().0;
        let bag_name = &line[0..second_space];
        names.push(bag_name.to_string());
    }
    names.shrink_to_fit();
    names
}

fn parse_bag_rules<T: Deref<Target = str>>(input: &[T], names: &[String]) -> Vec<Bag> {
    let mut bags = Vec::new();
    for line in input {
        let mut stream = uwl::Stream::new(line);
        let second_space = line.match_indices(' ').nth(1).unwrap().0;
        let name = stream.advance(second_space);
        let id = names.iter().position(|s| s == name).unwrap();
        // Eat the space + "bags contain" + space
        stream.advance(14);
        let mut containment_index = 0;
        let mut rules: [Option<ContainmentRule>; 8] = [None; MAX_CONTAINED_BAGS];
        while !stream.is_empty() {
            let part = stream.peek_until(|c| c == b',' || c == b'.');
            let rule = if part == "no other bags" {
                None
            } else {
                let count = stream
                    .take_while(|c| c.is_ascii_digit())
                    .parse::<u8>()
                    .unwrap();
                stream.advance(1);
                let second_space = stream.rest().match_indices(' ').nth(1).unwrap().0;
                let name = stream.advance(second_space);
                let id = names.iter().position(|s| s == name).unwrap();
                Some(ContainmentRule { count, id })
            };
            rules[containment_index] = rule;
            // Eat the space + "bags" + comma + space
            stream.take_while(|c| c != b',');
            stream.advance(2);
            containment_index += 1;
        }
        bags.push(Bag {
            id,
            contains: rules,
        })
    }
    bags.sort_unstable_by_key(|c| c.id);
    bags
}

#[cfg(test)]
const TEST_INPUT: &str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[test]
fn test_id_map() {
    let lines: Vec<&str> = TEST_INPUT.trim().lines().map(|l| l.trim()).collect();
    let bag_names = get_bag_names(&lines);
    assert_eq!(bag_names.len(), 9);
    assert_eq!(
        bag_names,
        vec![
            "light red",
            "dark orange",
            "bright white",
            "muted yellow",
            "shiny gold",
            "dark olive",
            "vibrant plum",
            "faded blue",
            "dotted black"
        ]
    );
}

#[test]
fn test_parse_bags() {
    let lines: Vec<&str> = TEST_INPUT.trim().lines().map(|l| l.trim()).collect();
    let bag_names = get_bag_names(&lines);
    let bags = parse_bag_rules(&lines, &bag_names);
    assert_eq!(
        bags.get(0).unwrap(),
        &Bag {
            id: 0,
            contains: [
                Some(ContainmentRule { id: 2, count: 1 }),
                Some(ContainmentRule { id: 3, count: 2 }),
                None,
                None,
                None,
                None,
                None,
                None
            ]
        }
    );
    assert_eq!(
        bags.get(2).unwrap(),
        &Bag {
            id: 2,
            contains: [
                Some(ContainmentRule { id: 4, count: 1 }),
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]
        }
    );
    assert_eq!(
        bags.get(8).unwrap(),
        &Bag {
            id: 8,
            contains: [None; 8]
        }
    );
}

#[test]
fn test_can_contain() {
    let lines: Vec<&str> = TEST_INPUT.trim().lines().map(|l| l.trim()).collect();
    let bag_names = get_bag_names(&lines);
    let bags = parse_bag_rules(&lines, &bag_names);
    assert!(bags[0].can_contain(2, &bags));
    assert!(bags[0].can_contain(4, &bags));
    assert!(bags[0].can_contain(7, &bags));
}

#[test]
fn test_count_contained() {
    let lines: Vec<&str> = TEST_INPUT.trim().lines().map(|l| l.trim()).collect();
    let bag_names = get_bag_names(&lines);
    let bags = parse_bag_rules(&lines, &bag_names);
    let bag_id = bag_names.iter().position(|c| c == "shiny gold").unwrap();

    assert_eq!(bags[bag_id].count_contained(&bags) - 1, 32);
}

#[test]
fn test_available_colors() {
    let test_color = "shiny gold";
    let lines: Vec<&str> = TEST_INPUT.trim().lines().map(|l| l.trim()).collect();
    let bag_names = get_bag_names(&lines);
    let bags = parse_bag_rules(&lines, &bag_names);
    let bag_id = bag_names.iter().position(|c| c == test_color).unwrap();

    let count = bags.iter().filter(|b| b.can_contain(bag_id, &bags)).count();
    assert_eq!(count, 4);
}

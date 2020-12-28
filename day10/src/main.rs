use anyhow::Result;

fn main() -> Result<()> {
    let mut input: Vec<u32> = input::read_lines("inputs/10")?;
    input.sort_unstable();

    println!("part 1: {}", pt1(&input));

    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);
    input.push(input.last().unwrap() + 3);
    println!("part 2: {}", arrangements(&input));

    Ok(())
}

fn pt1(adapters: &[u32]) -> u32 {
    let c = adapters
        .iter()
        .fold((0, 1, 0_u32), |acc, a| match a - acc.2 {
            1 => (acc.0 + 1, acc.1, acc.2 + (a - acc.2)),
            3 => (acc.0, acc.1 + 1, acc.2 + (a - acc.2)),
            _ => (acc.0, acc.1, acc.2 + (a - acc.2)),
        });
    c.0 * c.1
}

fn arrangements(adapters: &[u32]) -> u64 {
    let mut arr = 1;
    let mut tmp = 0;
    adapters
        .windows(2)
        .for_each(|window| match window[1] - window[0] {
            1 => tmp += 1,
            3 => {
                tmp += 1;
                arr *= match tmp {
                    1 => 1,
                    2 => 1,
                    3 => 2,
                    4 => 4,
                    5 => 7,
                    _ => unreachable!(),
                };
                tmp = 0;
            }
            _ => (),
        });
    arr
}

#[cfg(test)]
const TEST_INPUT: [u32; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

#[cfg(test)]
const TEST_INPUT_LARGE: [u32; 31] = [
    28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17,
    7, 9, 4, 2, 34, 10, 3,
];

#[test]
fn test_pt1() {
    let mut input = Vec::from(TEST_INPUT);
    input.sort_unstable();
    assert_eq!(pt1(&input), 5 * 7);

    let mut input_large = Vec::from(TEST_INPUT_LARGE);
    input_large.sort_unstable();
    assert_eq!(pt1(&input_large), 22 * 10);
}

#[test]
fn test_arrangements() {
    let mut input = Vec::from(TEST_INPUT);
    input.sort_unstable();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);

    assert_eq!(arrangements(&input), 8);

    let mut input_large = Vec::from(TEST_INPUT_LARGE);
    input_large.sort_unstable();
    input_large.insert(0, 0);
    input_large.push(input_large.last().unwrap() + 3);

    assert_eq!(arrangements(&input_large), 19208);
}

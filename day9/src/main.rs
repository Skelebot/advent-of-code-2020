use std::cmp::Ordering;

use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<u64> = input::read_lines("inputs/9")?;
    let preamble = 25;

    let invalid = find_invalid(&input, preamble);
    println!("part 1: {}", invalid);
    println!("part 2: {}", find_weakness(&input, invalid));
    Ok(())
}

fn find_invalid(data: &[u64], p_len: usize) -> u64 {
    // Temporary preamble, used to sort every number's preamble
    let mut tmp_pre: Vec<u64>;
    // Sort the preamble
    for (i, num) in data[p_len..].iter().enumerate() {
        tmp_pre = data[i..(p_len + i)].into();
        if !is_sum_of_any(&mut tmp_pre, num) {
            return *num;
        }
    }
    unreachable!()
}

fn is_sum_of_any(input: &mut [u64], num: &u64) -> bool {
    input.sort_unstable();
    let mut front = 0;
    let mut back = input.len() - 1;

    loop {
        let t = input[front] + input[back];
        if t > *num {
            back -= 1;
        }
        if t < *num {
            front += 1;
            back = input.len() - 1;
        }
        if t == *num {
            break true;
        }
        if front >= back {
            break false;
        }
    }
}

fn find_weakness(data: &[u64], num: u64) -> u64 {
    let mut front = 1;
    let mut tail = 0;
    loop {
        let sum = data[tail..front].iter().sum::<u64>();
        match sum.cmp(&num) {
            Ordering::Greater => tail += 1,
            Ordering::Less => front += 1,
            Ordering::Equal => {
                let max = data[tail..front].iter().max().unwrap();
                let min = data[tail..front].iter().min().unwrap();
                break max + min;
            }
        }
    }
}

#[cfg(test)]
const TEST_INPUT: [u64; 20] = [
    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
];

#[test]
fn test_find_invalid() {
    assert_eq!(find_invalid(&TEST_INPUT, 5), 127);
}

#[test]
fn test_find_weakness() {
    assert_eq!(find_weakness(&TEST_INPUT, 127), 62);
}

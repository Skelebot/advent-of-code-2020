use anyhow::Result;

const TARGET: u16 = 2020;

fn main() -> Result<()> {
    let mut input: Vec<u16> = input::read_lines("inputs/1")?;

    input.sort_unstable();

    println!("part 1: {}", find_two(&input));
    println!("part 2: {}", find_three(&input));

    Ok(())
}

fn find_two(input: &[u16]) -> u32 {
    let mut front = 0;
    let mut back = input.len() - 1;

    loop {
        let t = input[front] + input[back];
        if t > TARGET {
            back -= 1;
        }
        if t < TARGET {
            front += 1;
            back = input.len() - 1;
        }
        if t == TARGET {
            break;
        }
    }
    input[front] as u32 * input[back] as u32
}

fn find_three(input: &[u16]) -> u32 {
    let mut third = 0;
    let mut front;
    let mut back;

    'outer: loop {
        let target = TARGET - input[third];
        if target == 0 {
            continue;
        }

        front = 0;
        back = input.len() - 1;
        loop {
            let t = input[front] + input[back];
            if t > target {
                back -= 1;
                if back == 0 || back == front {
                    break;
                }
            }
            if t < target {
                front += 1;
                back = input.len() - 1;
            }
            if t == target {
                break 'outer;
            }
        }
        third += 1;
    }

    input[front] as u32 * input[back] as u32 * input[third] as u32
}

#[test]
fn test_find_two() {
    let mut input: Vec<u16> = vec![1721, 979, 366, 299, 675, 1456];
    input.sort_unstable();
    assert_eq!(find_two(&input), 514579);
}

#[test]
fn test_find_three() {
    let mut input: Vec<u16> = vec![1721, 979, 366, 299, 675, 1456];
    input.sort_unstable();
    assert_eq!(find_three(&input), 241861950);
}

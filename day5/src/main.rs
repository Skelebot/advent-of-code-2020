use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = input::read_lines("inputs/5")?;
    let input: Vec<(&str, &str)> = input.iter().map(|s| s.split_at(7)).collect();

    let mut taken: [u8; 128] = [0; 128];
    let mut max_id = 0;
    for seat in input {
        let row = find_num(seat.0, 'F', 127);
        let col = find_num(seat.1, 'L', 7);
        let id = (row as u32 * 8) + col as u32;
        // part 1
        if id > max_id {
            max_id = id
        }
        // part 2
        taken[row as usize] |= 1 << col;
    }

    println!("part 1: {}", max_id);

    // find a 0 (empty seat)
    for (row, s) in taken.iter().enumerate() {
        if *s < u8::MAX && s.count_zeros() == 1 {
            for col in 0..8 {
                if (*s & (1 << col)) == 0 {
                    let id = (row * 8) + col;
                    println!("part 2: {}", id)
                }
            }
        }
    }

    Ok(())
}

fn find_num(desc: &str, lower: char, max: u8) -> u8 {
    let mut row = max;
    let mut div = max;
    for c in desc.chars() {
        div /= 2;
        if c == lower {
            row -= div + 1;
        }
    }
    row
}

#[test]
fn test_find_num() {
    let input = "FBFBBFF";
    let row = find_num(&input, 'F', 127);
    assert_eq!(row, 44);
    let input = "RLR";
    let column = find_num(&input, 'L', 7);
    assert_eq!(column, 5);
}

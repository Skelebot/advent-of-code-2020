use std::ops::Deref;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Field {
    Open,
    Tree,
}

fn main() -> Result<()> {
    let map = parse_map(&input::read_lines::<String, _>("inputs/3")?);

    println!("part 1: {}", count_hits(&map, &(3, 1)));

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut acc = 1;
    for s in slopes.iter() {
        let h = count_hits(&map, s);
        acc *= h;
    }
    println!(
        "part 2: {}",
        //slopes.iter().fold(1, |acc, i| acc * count_hits(&map, i))
        acc
    );

    Ok(())
}

fn count_hits(map: &[Vec<Field>], slope: &(usize, usize)) -> u32 {
    let width = map[0].len();

    let mut hits: u32 = 0;

    let mut x: usize = slope.0;
    for y in (slope.1..map.len()).step_by(slope.1) {
        if map[y][x] == Field::Tree {
            hits += 1;
        }
        x += slope.0;
        if x >= width {
            x -= width
        }
    }

    hits
}

fn parse_map<T: Deref<Target = str>>(input: &[T]) -> Vec<Vec<Field>> {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Field::Open,
                    '#' => Field::Tree,
                    _ => unreachable!(),
                })
                .collect::<Vec<Field>>()
        })
        .collect::<Vec<Vec<Field>>>()
}

#[test]
fn test_hits() {
    const TEST_INPUT: [&'static str; 11] = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    let map = parse_map(&TEST_INPUT);
    assert_eq!(count_hits(&map, &(1, 1)), 2);
    assert_eq!(count_hits(&map, &(3, 1)), 7);
    assert_eq!(count_hits(&map, &(5, 1)), 3);
    assert_eq!(count_hits(&map, &(7, 1)), 4);
    assert_eq!(count_hits(&map, &(1, 2)), 2);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    assert_eq!(
        slopes.iter().fold(1, |acc, i| acc * count_hits(&map, i)),
        336
    );
}

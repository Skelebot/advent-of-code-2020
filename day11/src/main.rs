use std::fmt::Display;

use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = input::read_lines("inputs/11")?;
    let mut automata = Automata::from(input);
    println!("part 1: {}", run_automata(&mut automata));
    println!("part 2: {}", 0);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Taken,
    Floor,
    Unspecified,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'L' => Cell::Empty,
            '.' => Cell::Floor,
            '#' => Cell::Taken,
            _ => panic!("Invalid cell: {}", c),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Empty => "L",
            Cell::Floor => ".",
            Cell::Taken => "#",
            _ => "?",
        })
    }
}

struct Automata {
    layout: Vec<Vec<Cell>>,
    next_buf: Vec<Vec<Cell>>,
}

impl Automata {
    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.layout[y + 1][x + 1]
    }

    pub fn count_neighbors(&self, x: usize, y: usize, ty: &Cell) -> usize {
        let mut c = 0;
        c += self.layout[y][x..=x + 2]
            .iter()
            .filter(|cell| *cell == ty)
            .count();
        if self.layout[y + 1][x] == *ty {
            c += 1
        }
        if self.layout[y + 1][x + 2] == *ty {
            c += 1
        }
        c += self.layout[y + 2][x..=x + 2]
            .iter()
            .filter(|cell| *cell == ty)
            .count();
        c
    }

    pub fn count_cells(&self, ty: &Cell) -> usize {
        self.layout.iter().flatten().filter(|c| *c == ty).count()
    }

    pub fn step(&mut self) -> bool {
        let mut modified = false;
        self.next_buf = self.layout.clone();
        for y in 0..self.layout.len() - 2 {
            for x in 0..self.layout[0].len() - 2 {
                match self.get_cell(x, y) {
                    Cell::Empty => {
                        if self.count_neighbors(x, y, &Cell::Taken) == 0 {
                            modified = true;
                            self.next_buf[y + 1][x + 1] = Cell::Taken
                        }
                    }
                    Cell::Taken => {
                        if self.count_neighbors(x, y, &Cell::Taken) >= 4 {
                            modified = true;
                            self.next_buf[y + 1][x + 1] = Cell::Empty
                        }
                    }
                    _ => {}
                }
            }
        }
        self.layout = self.next_buf.clone();
        modified
    }
}

impl From<Vec<String>> for Automata {
    fn from(i: Vec<String>) -> Self {
        let mut layout = Vec::with_capacity(i.len() + 2);
        for s in i {
            let mut row: Vec<Cell> = s.chars().map(Cell::from).collect();
            row.insert(0, Cell::Unspecified);
            row.push(Cell::Unspecified);
            layout.push(row);
        }
        layout.insert(0, vec![Cell::Unspecified; layout[0].len()]);
        layout.push(vec![Cell::Unspecified; layout[0].len()]);
        Automata {
            layout,
            next_buf: Vec::new(),
        }
    }
}

impl Display for Automata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 1..self.layout.len() - 2 {
            for x in 1..self.layout[y].len() - 2 {
                f.write_fmt(format_args!("{}", self.layout[y][x]))?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

fn run_automata(automata: &mut Automata) -> usize {
    while automata.step() {}
    automata.count_cells(&Cell::Taken)
}

#[test]
fn test_run_automata() {
    let mut automata = Automata::from(vec![
        "L.LL.LL.LL".to_string(),
        "LLLLLLL.LL".to_string(),
        "L.L.L..L..".to_string(),
        "LLLL.LL.LL".to_string(),
        "L.LL.LL.LL".to_string(),
        "L.LLLLL.LL".to_string(),
        "..L.L.....".to_string(),
        "LLLLLLLLLL".to_string(),
        "L.LLLLLL.L".to_string(),
        "L.LLLLL.LL".to_string(),
    ]);

    assert_eq!(run_automata(&mut automata), 37);
}

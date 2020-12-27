use std::ops::Deref;

fn main() -> anyhow::Result<()> {
    let input: Vec<String> = input::read_lines("inputs/8")?;
    let asm: Vec<Instruction> = parse_asm(&input);

    println!("part 1: {}", run_asm_looping(&asm));

    println!("part 2: {}", run_asm_fix(&asm));
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn run_asm_looping(code: &[Instruction]) -> i32 {
    let mut ran = vec![false; code.len()];
    let mut acc = 0;
    let mut pointer = 0;
    loop {
        let ran = &mut ran[pointer];
        if *ran {
            break;
        }
        *ran = true;
        let ins = &code[pointer];
        match *ins {
            Instruction::Nop(_) => {}
            Instruction::Acc(val) => acc += val,
            Instruction::Jmp(val) => {
                if val < 0 {
                    pointer -= -val as usize
                } else {
                    pointer += val as usize
                }
                // Do not advance the pointer
                continue;
            }
        };
        pointer += 1;
    }
    acc
}

fn run_asm_fix(code: &[Instruction]) -> i32 {
    let mut ran = vec![false; code.len()];
    let mut acc = 0;
    let mut pointer = 0;
    loop {
        let has_ran = &mut ran[pointer];
        if *has_ran {
            break;
        }
        *has_ran = true;
        let ins = &code[pointer];
        match *ins {
            Instruction::Nop(val) => {
                // This would create a single-instruction loop
                if val != 0 {
                    // Try to jump
                    pointer = (pointer as isize + val as isize) as usize;
                    if let Ok(result_acc) = try_finish_run(code, &ran, &pointer, &acc) {
                        return result_acc;
                    } else {
                        // Backtrack the jump
                        pointer = (pointer as isize - val as isize) as usize;
                    }
                }
            }
            Instruction::Acc(val) => acc += val,
            Instruction::Jmp(val) => {
                // Try to do nothing
                if let Ok(result_acc) = try_finish_run(code, &ran, &(pointer + 1), &acc) {
                    return result_acc;
                } else {
                    // Jump normally
                    pointer = (pointer as isize + val as isize) as usize;
                }
                // Do not advance the pointer
                continue;
            }
        };
        pointer += 1;
    }
    acc
}

fn try_finish_run(
    code: &[Instruction],
    ran: &[bool],
    pointer: &usize,
    acc: &i32,
) -> Result<i32, i32> {
    let mut ran = Vec::from(ran);
    let mut acc = *acc;
    let mut pointer = *pointer;
    loop {
        if pointer >= code.len() {
            return Ok(acc);
        }
        let ran = &mut ran[pointer];
        if *ran {
            return Err(acc);
        }
        *ran = true;
        let ins = &code[pointer];
        match *ins {
            Instruction::Nop(_) => {}
            Instruction::Acc(val) => acc += val,
            Instruction::Jmp(val) => {
                if val < 0 {
                    pointer -= -val as usize
                } else {
                    pointer += val as usize
                }
                // Do not advance the pointer
                continue;
            }
        };
        pointer += 1;
    }
}

fn parse_asm<T: Deref<Target = str>>(input: &[T]) -> Vec<Instruction> {
    let mut out = Vec::with_capacity(input.len());
    for line in input {
        let val = line[4..].parse::<i32>().unwrap();
        match &line[0..3] {
            "nop" => out.push(Instruction::Nop(val)),
            "acc" => out.push(Instruction::Acc(val)),
            "jmp" => out.push(Instruction::Jmp(val)),
            _ => (),
        }
    }
    out
}

#[cfg(test)]
const TEST_INPUT: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

#[test]
fn test_run_asm_looping() {
    let lines: Vec<String> = TEST_INPUT
        .trim()
        .lines()
        .map(|l| l.trim().to_string())
        .collect();
    let asm: Vec<Instruction> = parse_asm(&lines);
    let result = run_asm_looping(&asm);
    assert_eq!(result, 5);
}

#[test]
fn test_run_asm_fix() {
    let lines: Vec<String> = TEST_INPUT
        .trim()
        .lines()
        .map(|l| l.trim().to_string())
        .collect();
    let asm: Vec<Instruction> = parse_asm(&lines);
    let result = run_asm_fix(&asm);
    assert_eq!(result, 8);
}

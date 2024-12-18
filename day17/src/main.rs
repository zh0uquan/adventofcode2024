use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, u32 as nom_u32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use z3::ast::Bool;
use z3::{
    ast::{Ast, BV},
    Config, Context, Optimize, SatResult, Solver,
};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2());
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn new(opcode: u32) -> Self {
        match opcode {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("disco!"),
        }
    }
}

#[derive(Debug, Default)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    programs: Vec<u32>,
    pointer: usize,
    output: Vec<u32>,
    skip_increase: bool,
}

impl Computer {
    fn combo(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => {
                panic!("reserved!")
            }
            _ => panic!("disco!"),
        }
    }

    fn run(&mut self) -> bool {
        while self.pointer < self.programs.len() {
            let (opcode, operand) = self.programs
                [self.pointer..self.pointer + 2]
                .iter()
                .copied()
                .collect_tuple()
                .unwrap();
            self.op(opcode, operand);
            if !self.skip_increase {
                self.pointer += 2;
            }
            self.skip_increase = false;
        }
        self.output == self.programs
    }

    fn op(&mut self, opcode: u32, operand: u32) {
        let instruction = Instruction::new(opcode);
        match instruction {
            Instruction::Adv => {
                self.a /= 2_u32.pow(self.combo(operand));
            }
            Instruction::Bxl => {
                self.b ^= operand;
            }
            Instruction::Bst => {
                self.b = self.combo(operand) % 8;
            }
            Instruction::Jnz => {
                if self.a != 0 {
                    self.pointer = operand as usize;
                    self.skip_increase = true;
                }
            }
            Instruction::Bxc => {
                self.b ^= self.c;
            }
            Instruction::Out => {
                self.output.push(self.combo(operand) % 8);
            }
            Instruction::Bdv => {
                self.b = self.a / 2_u32.pow(self.combo(operand));
            }
            Instruction::Cdv => {
                self.c = self.a / 2_u32.pow(self.combo(operand));
            }
        };
    }
}

fn parse_register(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
    separated_list1(
        newline,
        preceded(tag("Register "), separated_pair(alpha1, tag(": "), nom_u32)),
    )(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("Program: "), separated_list1(tag(","), nom_u32))(input)
}

fn parse_computer(input: &str) -> Computer {
    let (register, program) = input.split("\n\n").collect_tuple().unwrap();
    let register = parse_register(register).unwrap().1;
    let instructions: Vec<u32> = parse_program(program).unwrap().1;

    Computer {
        a: register[0].1,
        b: register[1].1,
        c: register[2].1,
        programs: instructions,
        ..Computer::default()
    }
}

fn part1(input: &str) -> String {
    let mut computer = parse_computer(input);
    computer.run();
    computer.output.iter().join(",").to_string()
}

fn part2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    let n = BV::new_const(&ctx, "n", 64);

    let mut a = n.clone();
    let mut b = BV::from_u64(&ctx, 0, 64);
    let mut c = BV::from_u64(&ctx, 0, 64);

    let x_values = [2, 4, 1, 3, 7, 5, 4, 2, 0, 3, 1, 5, 5, 5, 3, 0];

    for x in x_values {
        // b = a % 8
        b = a.bvurem(&BV::from_u64(&ctx, 8, 64));

        // b = b ^ 3
        b = b.bvxor(&BV::from_u64(&ctx, 3, 64));

        // c = a / (1 << b)
        c = a.bvudiv(&BV::from_u64(&ctx, 1, 64).bvshl(&b));

        // b = b ^ c
        b = b.bvxor(&c);

        // a = a / 8
        a = a.bvudiv(&BV::from_u64(&ctx, 8, 64));

        // b = a ^ 5
        b = b.bvxor(&BV::from_u64(&ctx, 5, 64));

        // Add constraint that (b % 8) equals x
        opt.assert(
            &b.bvurem(&BV::from_u64(&ctx, 8, 64))
                ._eq(&BV::from_u64(&ctx, x as u64, 64)),
        );
    }

    // Add constraint that a == 0
    opt.assert(&a._eq(&BV::from_u64(&ctx, 0, 64)));

    // Minimize n
    opt.minimize(&n);

    // Check satisfiability
    match opt.check(&[]) {
        SatResult::Sat => {
            if let Some(model) = opt.get_model() {
                if let Some(n_value) = model.eval(&n, true) {
                    println!("Found value: {}", n_value.as_u64().unwrap());
                } else {
                    println!("Could not evaluate the value of 'n'.");
                }
            }
        }
        SatResult::Unsat => println!("Unsatisfiable"),
        SatResult::Unknown => println!("Unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_set_register_when_c_is_9() {
        let mut computer = Computer {
            c: 9,
            programs: vec![2, 6],
            ..Default::default()
        };

        computer.run();

        assert_eq!(computer.b, 1);
    }

    #[test]
    fn test_output_for_register_a_10() {
        let mut computer = Computer {
            a: 10,
            programs: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };

        computer.run();

        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_multiply_when_a_is_2024() {
        let mut computer = Computer {
            a: 2024,
            programs: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };

        computer.run();
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn test_subtract_b_from_b() {
        let mut computer = Computer {
            b: 29,
            programs: vec![1, 7],
            ..Default::default()
        };

        computer.run();

        assert_eq!(computer.b, 26);
    }

    #[test]
    fn test_set_register_b_with_b_and_c() {
        let mut computer = Computer {
            b: 2024,
            c: 43690,
            programs: vec![4, 0],
            ..Default::default()
        };

        computer.run();

        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
            "#
        };
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");

        let input1 = indoc! {
            r#"
            Register A: 117440
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0
            "#
        };

        assert_eq!(part1(input1), "0,3,5,4,3,0")
    }
}

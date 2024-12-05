advent_of_code::solution!(12);

// * `cpy x y` *copies* `x` (either an integer or the *value* of a register) into register `y`.
// * `inc x` *increases* the value of register `x` by one.
// * `dec x` *decreases* the value of register `x` by one.
// * `jnz x y` *jumps* to an instruction `y` away (positive means forward; negative means backward), but only if `x` is *not zero*.

pub enum Instruction<'a> {
    Cpy((&'a str, &'a str)),
    Inc(&'a str),
    Dec(&'a str),
    Jnz((&'a str, i32)),
}

pub struct State<'a> {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,

    pub pos: i32,
    pub instructions: Vec<Instruction<'a>>,
}

impl<'a> State<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>, c: i32) -> Self {
        Self {
            a: 0,
            b: 0,
            c,
            d: 0,

            pos: 0,
            instructions,
        }
    }

    pub fn run(&mut self) {
        let mut is_complete = false;
        while !is_complete {
            match self.instructions.get(self.pos as usize) {
                Some(Instruction::Cpy((v, d))) => {
                    let value = match *v {
                        "a" => self.a,
                        "b" => self.b,
                        "c" => self.c,
                        "d" => self.d,
                        v => v.parse::<i32>().unwrap(),
                    };
                    match *d {
                        "a" => self.a = value,
                        "b" => self.b = value,
                        "c" => self.c = value,
                        "d" => self.d = value,
                        v => panic!("Invalid register for cpy {v}"),
                    }
                    self.pos += 1;
                }
                Some(Instruction::Inc(v)) => {
                    match *v {
                        "a" => self.a += 1,
                        "b" => self.b += 1,
                        "c" => self.c += 1,
                        "d" => self.d += 1,
                        v => panic!("Invalid register: {v}"),
                    }
                    self.pos += 1;
                }
                Some(Instruction::Dec(v)) => {
                    match *v {
                        "a" => self.a -= 1,
                        "b" => self.b -= 1,
                        "c" => self.c -= 1,
                        "d" => self.d -= 1,
                        v => panic!("Invalid register: {v}"),
                    }
                    self.pos += 1;
                }
                Some(Instruction::Jnz((s, d))) => {
                    let should_skip = match *s {
                        "a" => self.a == 0,
                        "b" => self.b == 0,
                        "c" => self.c == 0,
                        "d" => self.d == 0,
                        v => v == "0",
                    };
                    if !should_skip {
                        self.pos += d;
                    } else {
                        self.pos += 1;
                    }
                }
                None => {
                    is_complete = true;
                }
            }
        }
    }
}

pub fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(' ').collect();
    match parts.first() {
        Some(&"cpy") => Instruction::Cpy((parts.get(1).unwrap(), parts.get(2).unwrap())),
        Some(&"inc") => Instruction::Inc(parts.get(1).unwrap()),
        Some(&"dec") => Instruction::Dec(parts.get(1).unwrap()),
        Some(&"jnz") => Instruction::Jnz((
            parts.get(1).unwrap(),
            parts.get(2).unwrap().parse::<i32>().unwrap(),
        )),

        _ => panic!("{} is not a recognized command", parts.first().unwrap()),
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(|l| parse_line(l)).collect();
    let mut state = State::new(instructions, 0);
    state.run();

    Some(state.a)
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(|l| parse_line(l)).collect();
    let mut state = State::new(instructions, 1);
    state.run();

    Some(state.a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}

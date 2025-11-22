#[derive(Debug, Clone)]
pub enum Instruction {
    Cpy((String, String)),
    Inc(String),
    Dec(String),
    Jnz((String, String)),
    Tgl(String),
}

pub struct State {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,

    pub pos: i32,
    pub instructions: Vec<Instruction>,
    pub debug: bool,
}

pub fn toggle_instruction(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::Cpy((a, b)) => Instruction::Jnz((a.to_owned(), b.to_owned())),
        Instruction::Inc(a) => Instruction::Dec(a.clone()),
        Instruction::Dec(a) => Instruction::Inc(a.clone()),
        Instruction::Jnz((a, b)) => Instruction::Cpy((a.to_string(), b.to_string())),
        Instruction::Tgl(a) => Instruction::Inc(a.to_owned()),
    }
}

impl State {
    pub fn new_c(instructions: Vec<Instruction>, c: i32) -> Self {
        Self {
            a: 0,
            b: 0,
            c,
            d: 0,

            pos: 0,
            instructions,
            debug: false,
        }
    }
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,

            pos: 0,
            instructions,
            debug: false,
        }
    }

    pub fn log(&self, msg: String) {
        if self.debug {
            println!("{}", msg);
        }
    }

    pub fn run(&mut self) {
        let mut is_complete = false;
        while !is_complete {
            // self.log(format!("{} {} {} {}", self.a, self.b, self.c, self.d));
            // self.log(format!("{:?}", self.instructions.get(self.pos as usize)));
            match self.instructions.get(self.pos as usize) {
                Some(Instruction::Cpy((v, d))) => {
                    let value = match v.as_str() {
                        "a" => self.a,
                        "b" => self.b,
                        "c" => self.c,
                        "d" => self.d,
                        v => v.parse::<i32>().unwrap(),
                    };
                    match d.as_str() {
                        "a" => self.a = value,
                        "b" => self.b = value,
                        "c" => self.c = value,
                        "d" => self.d = value,
                        _ => {}
                    }
                    self.pos += 1;
                }
                Some(Instruction::Inc(v)) => {
                    match v.as_str() {
                        "a" => self.a += 1,
                        "b" => self.b += 1,
                        "c" => self.c += 1,
                        "d" => self.d += 1,
                        v => panic!("Invalid register: {v}"),
                    }
                    self.pos += 1;
                }
                Some(Instruction::Dec(v)) => {
                    match v.as_str() {
                        "a" => self.a -= 1,
                        "b" => self.b -= 1,
                        "c" => self.c -= 1,
                        "d" => self.d -= 1,
                        v => panic!("Invalid register: {v}"),
                    }
                    self.pos += 1;
                }
                Some(Instruction::Jnz((s, d))) => {
                    let should_skip = match s.as_str() {
                        "a" => self.a == 0,
                        "b" => self.b == 0,
                        "c" => self.c == 0,
                        "d" => self.d == 0,
                        v => v == "0",
                    };
                    let d = match d.as_str() {
                        "a" => Some(self.a),
                        "b" => Some(self.b),
                        "c" => Some(self.c),
                        "d" => Some(self.d),
                        d => d.parse::<i32>().ok(),
                    };
                    if let Some(d) = d {
                        if !should_skip {
                            self.pos += d;
                        } else {
                            self.pos += 1;
                        }
                    } else {
                        self.pos += 1;
                    }
                }
                Some(Instruction::Tgl(dist)) => {
                    let dist = match dist.as_str() {
                        "a" => self.a,
                        "b" => self.b,
                        "c" => self.c,
                        "d" => self.d,
                        dist => panic!("Invalid register: {dist}"),
                    };
                    let index_to_modify = self.pos + dist;

                    if let Some(instruction) = self.instructions.get_mut(index_to_modify as usize) {
                        *instruction = toggle_instruction(instruction);
                        if self.debug {
                            println!("new instruction: {:?}", instruction);
                        }
                    }
                    self.pos += 1;
                }
                None => {
                    is_complete = true;
                }
            }
            // self.log(format!("{} {} {} {}\n", self.a, self.b, self.c, self.d));
        }
    }
}

pub fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(' ').collect();
    match parts.first() {
        Some(&"cpy") => Instruction::Cpy((
            parts.get(1).unwrap().to_string(),
            parts.get(2).unwrap().to_string(),
        )),
        Some(&"inc") => Instruction::Inc(parts.get(1).unwrap().to_string()),
        Some(&"dec") => Instruction::Dec(parts.get(1).unwrap().to_string()),
        Some(&"tgl") => Instruction::Tgl(parts.get(1).unwrap().to_string()),
        Some(&"jnz") => Instruction::Jnz((
            parts.get(1).unwrap().to_string(),
            parts.get(2).unwrap().to_string(),
        )),

        _ => panic!("{} is not a recognized command", parts.first().unwrap()),
    }
}

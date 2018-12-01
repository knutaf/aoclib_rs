use super::*;
use std::fmt;

pub enum RegisterOrValue {
    Reg(char),
    Val(i64),
}

pub enum Instruction {
    Snd(RegisterOrValue),
    Set(char, RegisterOrValue),
    Add(char, RegisterOrValue),
    Sub(char, RegisterOrValue),
    Mul(char, RegisterOrValue),
    Mod(char, RegisterOrValue),
    Rcv(char),
    Jgz(RegisterOrValue, RegisterOrValue),
    Jnz(RegisterOrValue, RegisterOrValue),
}

pub struct Program {
    pub instructions : Vec<Instruction>,
}

#[derive(Default)]
pub struct RegisterHolder {
    registers : [i64 ; ((b'z' - b'a') + 1) as usize],
}

impl RegisterOrValue {
    pub fn from(input : &str) -> RegisterOrValue {
        lazy_static! {
            static ref RE_REGISTER : regex::Regex = Regex::new(r"^([a-zA-Z])$").expect("failed to compile regex");
            static ref RE_VALUE : regex::Regex = Regex::new(r"^(-?\d+)$").expect("failed to compile regex");
        }

        if let Some(captures) = RE_REGISTER.captures_iter(input).next() {
            RegisterOrValue::Reg(captures.get(1).unwrap().as_str().chars().nth(0).unwrap())
        } else if let Some(captures) = RE_VALUE.captures_iter(input).next() {
            RegisterOrValue::Val(captures.get(1).unwrap().as_str().parse::<i64>().unwrap())
        } else {
            panic!("invalid register or value {}", input);
        }
    }
}

impl fmt::Display for RegisterOrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterOrValue::Reg(a) => write!(f, "{}", a),
            RegisterOrValue::Val(a) => write!(f, "{}", a),
        }
    }
}

impl Instruction {
    pub fn from(input : &str) -> Instruction {
        lazy_static! {
            static ref RE_SND : regex::Regex = Regex::new(r"^\s*snd (.*)$").expect("failed to compile regex");
            static ref RE_SET : regex::Regex = Regex::new(r"^\s*set ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_ADD : regex::Regex = Regex::new(r"^\s*add ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_SUB : regex::Regex = Regex::new(r"^\s*sub ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_MUL : regex::Regex = Regex::new(r"^\s*mul ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_MOD : regex::Regex = Regex::new(r"^\s*mod ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_RCV : regex::Regex = Regex::new(r"^\s*rcv ([a-zA-Z])$").expect("failed to compile regex");
            static ref RE_JGZ : regex::Regex = Regex::new(r"^\s*jgz (.*) (.*)$").expect("failed to compile regex");
            static ref RE_JNZ : regex::Regex = Regex::new(r"^\s*jnz (.*) (.*)$").expect("failed to compile regex");
        }

        if let Some(captures) = RE_SND.captures_iter(input).next() {
            Instruction::Snd(RegisterOrValue::from(captures.get(1).unwrap().as_str()))
        } else if let Some(captures) = RE_SET.captures_iter(input).next() {
            Instruction::Set(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_ADD.captures_iter(input).next() {
            Instruction::Add(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_SUB.captures_iter(input).next() {
            Instruction::Sub(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_MUL.captures_iter(input).next() {
            Instruction::Mul(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_MOD.captures_iter(input).next() {
            Instruction::Mod(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_JGZ.captures_iter(input).next() {
            Instruction::Jgz(RegisterOrValue::from(captures.get(1).unwrap().as_str()), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_JNZ.captures_iter(input).next() {
            Instruction::Jnz(RegisterOrValue::from(captures.get(1).unwrap().as_str()), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_RCV.captures_iter(input).next() {
            Instruction::Rcv(captures.get(1).unwrap().as_str().chars().nth(0).unwrap())
        } else {
            panic!("invalid move {}", input);
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Snd(ref a) => write!(f, "snd {}", a),
            Instruction::Set(ref a, ref b) => write!(f, "set {} {}", a, b),
            Instruction::Add(ref a, ref b) => write!(f, "add {} {}", a, b),
            Instruction::Sub(ref a, ref b) => write!(f, "sub {} {}", a, b),
            Instruction::Mul(ref a, ref b) => write!(f, "mul {} {}", a, b),
            Instruction::Mod(ref a, ref b) => write!(f, "mod {} {}", a, b),
            Instruction::Rcv(ref a) => write!(f, "rcv {}", a),
            Instruction::Jgz(ref a, ref b) => write!(f, "jgz {} {}", a, b),
            Instruction::Jnz(ref a, ref b) => write!(f, "jnz {} {}", a, b),
        }
    }
}

impl Program {
    pub fn load(input : &str) -> Program {
        Program {
            instructions : input.lines().map(Instruction::from).collect(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = write!(f, "");
        for inst in &self.instructions {
            ret = writeln!(f, "{}", inst);
        }
        ret
    }
}

impl RegisterHolder {
    pub fn get_reg_mut(&mut self, reg : char) -> &mut i64 {
        &mut self.registers[reg as usize - 'a' as usize]
    }

    pub fn get_reg(&self, reg : char) -> &i64 {
        &self.registers[reg as usize - 'a' as usize]
    }

    pub fn evaluate(&self, rv : &RegisterOrValue) -> i64 {
        match *rv {
            RegisterOrValue::Reg(r) => {
                *self.get_reg(r)
            },
            RegisterOrValue::Val(v) => {
                v
            }
        }
    }

    pub fn apply_instruction(&mut self, instruction : &Instruction) -> bool {
        match *instruction {
            Instruction::Set(ref reg, ref rv) => {
                eprintln!("  {} <= {}", reg, self.evaluate(&rv));
                *self.get_reg_mut(*reg) = self.evaluate(&rv);
                true
            },
            Instruction::Add(ref reg, ref rv) => {
                eprintln!("  add {} ({}) {} => {}", *reg, *self.get_reg(*reg), self.evaluate(&rv), *self.get_reg(*reg) + self.evaluate(&rv));
                *self.get_reg_mut(*reg) = *self.get_reg(*reg) + self.evaluate(&rv);
                true
            },
            Instruction::Sub(ref reg, ref rv) => {
                eprintln!("  sub {} ({}) {} => {}", *reg, *self.get_reg(*reg), self.evaluate(&rv), *self.get_reg(*reg) - self.evaluate(&rv));
                *self.get_reg_mut(*reg) = *self.get_reg(*reg) - self.evaluate(&rv);
                true
            },
            Instruction::Mul(ref reg, ref rv) => {
                eprintln!("  mul {} ({}) {} => {}", *reg, *self.get_reg(*reg), self.evaluate(&rv), *self.get_reg(*reg) * self.evaluate(&rv));
                *self.get_reg_mut(*reg) = *self.get_reg(*reg) * self.evaluate(&rv);
                true
            },
            Instruction::Mod(ref reg, ref rv) => {
                eprintln!("  mod {} ({}) {} => {}", *reg, *self.get_reg(*reg), self.evaluate(&rv), *self.get_reg(*reg) % self.evaluate(&rv));
                *self.get_reg_mut(*reg) = *self.get_reg(*reg) % self.evaluate(&rv);
                true
            },
            _ => false,
        }
    }

    pub fn get_next_ip(&self, instruction : &Instruction, current_ip : usize) -> usize {
        let offset = match *instruction {
            Instruction::Set(..) |
            Instruction::Add(..) |
            Instruction::Sub(..) |
            Instruction::Mul(..) |
            Instruction::Snd(..) |
            Instruction::Rcv(..) |
            Instruction::Mod(..) => {
                1
            },
            Instruction::Jgz(ref cond, ref jump_offset) => {
                let offset;
                let did;
                if self.evaluate(&cond) > 0 {
                    did = true;
                    offset = self.evaluate(&jump_offset);
                } else {
                    did = false;
                    offset = 1;
                }

                eprintln!("  jgz {} ({}) {} ({}) => {}", *cond, self.evaluate(cond), *jump_offset, self.evaluate(jump_offset), did);
                offset
            },
            Instruction::Jnz(ref cond, ref jump_offset) => {
                let offset;
                let did;
                if self.evaluate(&cond) != 0 {
                    did = true;
                    offset = self.evaluate(&jump_offset);
                } else {
                    did = false;
                    offset = 1;
                }

                eprintln!("  jnz {} ({}) {} ({}) => {}", *cond, self.evaluate(cond), *jump_offset, self.evaluate(jump_offset), did);
                offset
            },
        };

        if offset >= 0 {
            current_ip + (offset as usize)
        } else if (-offset as usize) <= current_ip {
            ((current_ip as i64) + offset) as usize
        } else {
            usize::max_value()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_print() {
        let input =
r"snd z
snd -10
set a z
set a -10
add a z
add a -10
sub a z
sub a -10
mul a z
mul a -10
mod a z
mod a -10
rcv z
jgz a z
jgz -10 -10
jgz a -10
jgz -10 a";
        assert_eq!(input, format!("{}", Program::load(&input)).trim());
    }

    #[test]
    fn store_load() {
        let mut holder = RegisterHolder::default();
        for i in ('a' as u8) .. ('z' as u8)+1 {
            *holder.get_reg_mut(char::from(i)) = i64::from(i);
        }

        for i in ('a' as u8) .. ('z' as u8)+1 {
            assert_eq!(*holder.get_reg(char::from(i)), i64::from(i));
        }
    }

    #[test]
    fn store_load_inst() {
        let mut holder = RegisterHolder::default();

        holder.apply_instruction(&Instruction::Set('a', RegisterOrValue::Val(i64::from('a' as u8))));

        for i in ('b' as u8) .. ('z' as u8)+1 {
            holder.apply_instruction(&Instruction::Set(char::from(i), RegisterOrValue::Reg(char::from(i - 1))));
            holder.apply_instruction(&Instruction::Add(char::from(i), RegisterOrValue::Val(1)));
        }

        for i in ('a' as u8) .. ('z' as u8)+1 {
            assert_eq!(*holder.get_reg(char::from(i)), i64::from(i));
        }
    }

    #[test]
    fn jumps() {
        let holder = RegisterHolder::default();
        assert_eq!(holder.get_next_ip(&Instruction::Set('a', RegisterOrValue::Val(1)), 0), 1);
        assert_eq!(holder.get_next_ip(&Instruction::Jgz(RegisterOrValue::Val(1), RegisterOrValue::Val(-10)), 0), usize::max_value());
        assert_eq!(holder.get_next_ip(&Instruction::Jnz(RegisterOrValue::Val(1), RegisterOrValue::Val(-10)), 0), usize::max_value());
    }

    #[test]
    fn jgz() {
        let mut holder = RegisterHolder::default();
        *holder.get_reg_mut('a') = 1;
        assert_eq!(holder.get_next_ip(&Instruction::Jgz(RegisterOrValue::Val(0), RegisterOrValue::Val(2)), 0), 1);
        assert_eq!(holder.get_next_ip(&Instruction::Jgz(RegisterOrValue::Val(1), RegisterOrValue::Val(2)), 0), 2);
        assert_eq!(holder.get_next_ip(&Instruction::Jgz(RegisterOrValue::Reg('a'), RegisterOrValue::Val(2)), 0), 2);
        assert_eq!(holder.get_next_ip(&Instruction::Jgz(RegisterOrValue::Reg('b'), RegisterOrValue::Val(2)), 0), 1);
    }

    #[test]
    fn jnz() {
        let mut holder = RegisterHolder::default();
        *holder.get_reg_mut('a') = 1;
        assert_eq!(holder.get_next_ip(&Instruction::Jnz(RegisterOrValue::Val(0), RegisterOrValue::Val(2)), 0), 1);
        assert_eq!(holder.get_next_ip(&Instruction::Jnz(RegisterOrValue::Val(1), RegisterOrValue::Val(2)), 0), 2);
        assert_eq!(holder.get_next_ip(&Instruction::Jnz(RegisterOrValue::Reg('a'), RegisterOrValue::Val(2)), 0), 2);
        assert_eq!(holder.get_next_ip(&Instruction::Jnz(RegisterOrValue::Reg('b'), RegisterOrValue::Val(2)), 0), 1);
    }
}

use std::{ops::BitXor, u64};

mod data;
mod tests;

fn main() {
    println!("Hello, world!");
    let mut uc = Ucontrol {
        reg_a: 55593699,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("2413750315445530"),
        output: String::from(""),
    };
    uc.eval();
    println!("{}", uc.output);
}

struct Ucontrol {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    i_p: usize,
    prog: String,
    output: String,
}

impl Ucontrol {
    fn eval(&mut self) -> &mut Ucontrol {
        while self.i_p < self.prog.len() {
            let literal = self
                .prog
                .chars()
                .nth(self.i_p + 1)
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap();
            let combo = match literal {
                0u64 => 0u64,
                1u64 => 1,
                2u64 => 2,
                3u64 => 3,
                4u64 => self.reg_a,
                5u64 => self.reg_b,
                6u64 => self.reg_c,
                7u64 => 7,
                _ => u64::MAX,
            };
            assert!(combo != u64::MAX);

            match self
                .prog
                .chars()
                .nth(self.i_p)
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap()
            {
                0u64 => {
                    self.reg_a = self.reg_a / 2u64.pow(combo.try_into().unwrap());
                    self.i_p += 2;
                }
                1u64 => {
                    self.reg_b = self.reg_b ^ literal;
                    self.i_p += 2;
                }
                2u64 => {
                    self.reg_b = combo % 8;
                    self.i_p += 2;
                }
                3u64 => {
                    if self.reg_a != 0 {
                        self.i_p = literal as usize;
                    } else {
                        self.i_p += 2;
                    }
                }
                4u64 => {
                    self.reg_b = self.reg_b.bitxor(self.reg_c);
                    self.i_p += 2;
                }
                5u64 => {
                    self.i_p += 2;
                    if self.output.len() != 0 {
                        self.output.push(',');
                    }
                    self.output.push_str((combo % 8).to_string().as_str());
                }
                6u64 => {
                    self.reg_b = self.reg_a / 2u64.pow(combo.try_into().unwrap());
                    self.i_p += 2;
                }
                7u64 => {
                    self.reg_c = self.reg_a / 2u64.pow(combo.try_into().unwrap());
                    self.i_p += 2;
                }
                _ => assert!(false),
            }
        }
        return self;
    }
}

use std::{collections::HashMap, ops::BitXor, u32, u64};

mod data;
mod tests;

fn main() {
    println!("Hello, world!");
    let mut uc: Ucontrol = Ucontrol {
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("2413750315445530"),
        output: String::from(""),
    };
    let mut ii: u64 = 0;
    while ii < 100 {
        uc.clear();
        uc.reg_a = ii;
        uc.eval();
        ii += 1;
        println!("{:}\t{}", ii,  uc.output);
    }
}

fn brutal() {
    let mut uc: Ucontrol = Ucontrol {
        reg_a: 16 * 3,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("2413750315445530"),
        output: String::from(""),
    };
    let mut ii: u64 = 0;
    let target = uc.prog.len();
    let prog = uc.prog.clone();
    for _c in prog.chars() {
        let len = uc.output.len();
        let mut ans = u64::MAX;
        while ans == ans {
            while uc.output.chars().nth(0).is_none() {
                uc.clear();
                uc.reg_a = ii;
                uc.eval();
                ii += 1;
            }
            if uc.output.chars().nth(0).unwrap() == _c {
                ans = ii;
            }
        }
        println!("{:}\t{}{}", ii, _c, uc.output);
    }
}
#[derive(Clone, Debug)]
struct Ucontrol {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    i_p: usize,
    prog: String,
    output: String,
}

impl Ucontrol {
    fn clear(&mut self) -> &mut Ucontrol {
        self.reg_a = 0;
        self.reg_b = 0;
        self.reg_c = 0;
        self.i_p = 0;
        self.output = String::from("");
        return self;
    }
    fn done(&mut self) -> bool {
        return self.i_p >= self.prog.len();
    }
    fn eval(&mut self) -> &mut Ucontrol {
        while self.i_p < self.prog.len() {
            print!("{} -> ", self.reg_a);
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
            println!("{}", self.reg_a);
        }
        return self;
    }
}

struct Hacker {
    trackers_0: HashMap<String, Ucontrol>,
}

impl Hacker {
    fn back_track(uc: &mut Ucontrol) -> Ucontrol {
        let mut _nuc = uc.clone();
        while _nuc.eval().output == uc.output {}
        return _nuc;
    }
}

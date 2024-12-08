use enum_iterator::Sequence;

use strum_macros::EnumIter;
extern crate num_derive;
extern crate num_traits;

use regex::Regex;

#[test]
fn test_007() {
    let mut calculon = CalculatorT::new(6);

    let eq = Equation::parse("3267: 81 40 27");
    assert!(eq.is_some());
    let res = eq.unwrap();
    assert_eq!(res, Equation::new(3267, &vec![81, 40, 27]));
    assert_eq!(res.test(&mut calculon).expect("aaaa"), true);

    let eq = Equation::parse("190: 10 19");
    assert!(eq.is_some());
    let res = eq.unwrap();
    assert_eq!(res, Equation::new(190, &vec![10, 19]));
    assert_eq!(res.test(&mut calculon).expect("aaaa"), true);

    let eq = Equation::parse("292: 11 6 16 20");
    assert_eq!(eq.unwrap().test(&mut calculon).expect("aaaa"), true);

    let eq = Equation::parse("156: 15 6");
    assert_eq!(eq.unwrap().test(&mut calculon).expect("aaaa"), true);

    let eq = Equation::parse("7290: 6 8 6 15");
    assert_eq!(eq.unwrap().test(&mut calculon).expect("aaaa"), true);
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Equation {
    operands: Vec<u64>,
    result: u64,
}

pub struct CalculatorT { 
    permut: Vec<OperatorT>,
}

pub fn next(vec: &mut Vec<OperatorT>) -> Option<&Vec<OperatorT>> {
    let mut ii: usize = 0;
    while vec[ii].next() {
        ii += 1;
        if ii >= vec.len(){
            return None;
        }
    }
    return Some(vec);
}

impl CalculatorT {
    pub fn new(_len: usize) -> CalculatorT {
        CalculatorT { 
            permut: Vec::new(),
        }
    }

    pub fn combinator(&mut self, size: usize) -> &Vec<OperatorT> {
        while self.permut.len() < size {
            self.permut.push(OperatorT::new(OperationT::Add));
        }
        while self.permut.len() > size {
            self.permut.pop();
        }
        for p in &mut self.permut {
            p.operation = OperationT::Add;
            p.name = "+".to_string();
        }
        return &self.permut;
    }
}

impl Equation {
    pub fn new(res: u64, ops: &Vec<u64>) -> Equation {
        Equation {
            result: res,
            operands: ops.clone(),
        }
    }
    pub fn get_result(&self) -> u64 {
        return self.result;
    }
    pub fn operands_cnt(input: &str) -> usize {
        let r_opr = Regex::new(r"(?<op>\d{1,})").unwrap();
        let mut operands: Vec<u64> = Vec::new();
        for s in input.split(" ").skip(1) {
            if r_opr.captures(s).is_some() {
                let cap = r_opr.captures(s).unwrap();
                if cap.get(1).is_some() {
                    operands.push(
                        cap.get(1)
                            .unwrap()
                            .as_str()
                            .parse::<u64>()
                            .expect("Parse error"),
                    );
                }
            }
        }
        return operands.len();
    }
    pub fn parse(input: &str) -> Option<Equation> {
        let r_res = Regex::new(r"(?<res>\d{1,}):").unwrap();
        let r_opr = Regex::new(r"(?<op>\d{1,})").unwrap();
        let mut cap = r_res.captures(input).unwrap();
        let result = cap
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .expect("Parse error");

        let mut operands: Vec<u64> = Vec::new();
        for s in input.split(" ").skip(1) {
            if r_opr.captures(s).is_some() {
                cap = r_opr.captures(s).unwrap();
                if cap.get(1).is_some() {
                    operands.push(
                        cap.get(1)
                            .unwrap()
                            .as_str()
                            .parse::<u64>()
                            .expect("Parse error"),
                    );
                }
            }
        }
        Some(Equation {
            result: result,
            operands: operands,
        })
    }

    pub fn test(&self, calc: &mut CalculatorT) -> Option<bool> {
        let size: usize = (self.operands.len() as usize) - 1;

        let mut op: Vec<OperatorT> = calc.combinator(size).clone();

        let mut cont: bool = true;
        while cont {
            let mut carry: u64 = self.operands[0];
            for _ii in 0..self.operands.len() - 1 {
                carry = op[_ii].eq(carry, self.operands[_ii + 1]);
            }
            if carry == self.result {
                return Some(true);
            }
            cont = next(&mut op).is_some();
        } 
        return Some(false);
    }
}

#[derive(PartialEq, Eq, Sequence, Debug, Copy, Clone, EnumIter, Hash)]
pub enum OperationT {
    Add = 0,
    Mult,
    Conc,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct OperatorT {
    name: String,
    operation: OperationT,
}

impl OperatorT {
    pub fn new(oprn: OperationT) -> OperatorT {
        let op_name = vec!["+".to_string(), "x".to_string(), "||".to_string()];
        OperatorT {
            name: op_name[oprn as usize].clone(),
            operation: oprn,
        }
    }
    pub fn eq(&self, a: u64, b: u64) -> u64 {
        let mut res = 0;
        match self.operation {
            OperationT::Add => res = a + b,
            OperationT::Mult => res = a * b,
            OperationT::Conc => {
                let mut s: String = a.to_string();
                s.push_str(b.to_string().as_str());
                res = s.parse::<u64>().expect("WTF");
            }
        }; 
        return res;
    }
    pub fn next(&mut self) -> bool {
        let is_last: bool = self.operation == OperationT::last().unwrap();
        let op_name = vec!["+".to_string(), "x".to_string(), "||".to_string()];
        if is_last {
            self.operation = OperationT::Add;
        } else {
            self.operation = Sequence::next(&self.operation).expect("WTF");
        }
        self.name = op_name[self.operation as usize].clone();
        return is_last;
    }
}

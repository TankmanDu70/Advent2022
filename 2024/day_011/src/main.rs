use core::num;
use std::collections::HashMap;
mod data;

#[test]
fn test_aoc() {
    let mut parser = Parser::new();
    assert_eq!(&parser.parse(data::TEST).numbers, &vec![125, 17]);

    assert_eq!(*parser.blink_map(6).numh.get(&2097446912).unwrap(), 1);
    assert_eq!(*parser.numh.get(&14168).unwrap(), 1);
    assert_eq!(*parser.numh.get(&4048).unwrap(), 1);
    assert_eq!(*parser.numh.get(&2).unwrap(), 4);
    assert_eq!(*parser.numh.get(&4).unwrap(), 1);
    assert_eq!(*parser.numh.get(&40).unwrap(), 2);
    assert_eq!(*parser.numh.get(&48).unwrap(), 2);
    assert_eq!(*parser.numh.get(&2024).unwrap(), 1);
    assert_eq!(*parser.numh.get(&80).unwrap(), 1);
    assert_eq!(*parser.numh.get(&96).unwrap(), 1);
    assert_eq!(*parser.numh.get(&0).unwrap(), 2);
    assert_eq!(*parser.numh.get(&8).unwrap(), 1);
    assert_eq!(*parser.numh.get(&6).unwrap(), 2);
    assert_eq!(*parser.numh.get(&7).unwrap(), 1);

    assert_eq!(
        &parser.clear().parse(data::DATA).numbers,
        &vec![41078, 18, 7, 0, 4785508, 535256, 8154, 447]
    );
    assert_eq!(&parser.clear().parse(data::TEST1).numbers, &vec![0]); 
    assert_eq!(parser.clear().parse(data::DATA).blink_map(25).sum(), 217443);
    assert_eq!(parser.clear().parse(data::DATA).blink_map(75).sum(), 257246536026785);
}

fn main() {
    println!("Hello, world!");
    let mut parser: Parser = Parser::new();
    parser.parse(data::DATA);
    parser.blink_map(75);
    println!("Result is {}", parser.sum());
}

#[derive(Clone, Debug)]
struct Parser {
    numbers: Vec<u64>,
    blinks: usize,
    hist: HashMap<u64, Vec<u64>>,
    numh: HashMap<u64, usize>,
    len: u32,
}
impl Parser {
    pub fn new() -> Parser {
        let numbers = Vec::new();
        let hm = HashMap::<u64, Vec<u64>>::new();
        let nhm = HashMap::<u64, usize>::new();
        Parser {
            numbers: numbers,
            blinks: 0,
            hist: hm,
            numh: nhm,
            len: 0,
        }
    }

    pub fn parse(&mut self, line: &str) -> &mut Parser {
        for l in line.split(" ") {
            self.numbers.push(l.parse::<u64>().unwrap());
        }
        for bv in &self.numbers {
            if self.numh.contains_key(&bv) {
                *self.numh.get_mut(&bv).unwrap() += 1;
            } else {
                self.numh.insert(*bv, 1usize);
            }
        }
        return self;
    }

    pub fn clear(&mut self) -> &mut Parser {
        self.numbers.clear();
        self.numh.clear();
        self.blinks = 0;
        return self;
    }

    pub fn blink_until(&mut self, times: usize) -> &mut Parser {
        let mut rep = times;
        let mut inserted = false;
        while rep != 0 {
            self.blinks += 1;
            let mut last = self.numbers.len();
            let mut n: usize = 0;
            while n != last {
                if inserted {
                    inserted = false;
                    n += 1;
                    continue;
                }
                let s = self.numbers[n].to_string();
                if self.numbers[n] == 0 && false {
                    //self.numbers[n] = 1;
                    self.numbers.remove(n);
                    last -= 1;
                    continue;
                } else if self.numbers[n] == 0 {
                    self.numbers[n] = 1;
                } else if s.len() % 2 == 0 {
                    let split: (&str, &str) = s.split_at(s.len() / 2);
                    self.numbers[n] = split.0.parse::<u64>().unwrap();
                    self.numbers.insert(n + 1, split.1.parse::<u64>().unwrap());
                    inserted = true;
                    last += 1;
                } else {
                    self.numbers[n] = self.numbers[n] * 2024;
                }
                n += 1;
            }
            rep -= 1;
        }
        //println!("After {} blink: \t {:?}", self.blinks, self.numbers);
        return self;
    }

    fn blink(&mut self, bv: u64) -> Vec<u64> {
        *self.numh.get_mut(&bv).unwrap() -= 1;
        if self.hist.contains_key(&bv) {
            return self.hist.get(&bv).unwrap().clone();
        }
        let s = bv.to_string();
        let mut vec = Vec::new();
        if bv == 0 {
            vec.push(1);
        } else if s.len() % 2 == 0 {
            let split = s.split_at(s.len() / 2);
            vec.push(split.0.parse::<u64>().unwrap());
            vec.push(split.1.parse::<u64>().unwrap());
        } else {
            vec.push(bv * 2024);
        }
        self.hist.insert(bv, vec.clone());
        return vec;
    }

    pub fn blink_map(&mut self, times: usize) -> &mut Parser {
        let mut rep = times;

        while rep != 0 {
            self.blinks += 1;
            let existing = self.numh.clone();

            for it in existing {
                if self.hist.contains_key(&it.0) {
                    for value in self.hist.get(&it.0).unwrap() {
                        //history contains result as as flat vec
                        if self.numh.contains_key(value) {
                            *self.numh.get_mut(value).unwrap() += &it.1;
                        } else {
                            self.numh.insert(*value, it.1);
                        }
                    }
                } else {
                    let s = it.0.to_string();
                    let mut vec = Vec::new();
                    if it.0 == 0 {
                        vec.push(1);
                    } else if s.len() % 2 == 0 {
                        let split = s.split_at(s.len() / 2);
                        vec.push(split.0.parse::<u64>().unwrap());
                        vec.push(split.1.parse::<u64>().unwrap());
                    } else {
                        vec.push(it.0 * 2024);
                    }
                    self.hist.insert(it.0, vec.clone());
                    for value in &vec {
                        if self.numh.contains_key(value) {
                            *self.numh.get_mut(value).unwrap() += &it.1;
                        } else {
                            self.numh.insert(*value, it.1);
                        }
                    }
                }
                *self.numh.get_mut(&it.0).unwrap() -= it.1;
            }

            rep -= 1;
            println!("After {:3} blinks: {:32}", self.blinks.clone(), self.sum());
        }
        return self;
    }

    pub fn sum(&mut self) -> u64 {
        let mut ret = 0;
        for item in &mut self.numh.iter() {
            ret += *item.1 as u64;
        }
        return ret;
    }
}

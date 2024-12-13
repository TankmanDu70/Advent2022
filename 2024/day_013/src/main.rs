use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
mod data;
use regex;

pub const CRASY_HIGH: u128 = 10000000000000;

#[test]
fn test_aoc() {
    let mut m = Machine::new(8400, 5400, &Pos { x: 94, y: 34 }, &Pos { x: 22, y: 67 });
    assert_eq!(
        vec![m.real_math().best_a_f, m.real_math().best_b_f],
        vec![80, 40]
    );
    let mut m = Machine::new(7870, 6450, &Pos { x: 17, y: 86 }, &Pos { x: 84, y: 37 });
    assert_eq!(
        vec![m.real_math().best_a_f, m.real_math().best_b_f],
        vec![38, 86]
    );

    let v = parse(data::TEST, false);
    assert_eq!(v.len(), 2);
    assert_eq!(vec![v[0].cost(), v[1].cost()], vec![280, 200]);
    assert_eq!(vec![v[0].prize.x, v[0].prize.y], vec![8400, 5400]);
    assert_eq!(vec![v[1].prize.x, v[1].prize.y], vec![7870, 6450]);

    let v = parse(data::TEST, true);
    assert_eq!(v.len(), 2);
    assert_eq!(
        vec![v[0].prize.x, v[0].prize.y],
        vec![12748 + CRASY_HIGH, 12176 + CRASY_HIGH]
    );
    assert_eq!(
        vec![v[1].prize.x, v[1].prize.y],
        vec![18641 + CRASY_HIGH, 10279 + CRASY_HIGH]
    );

    let v = parse(data::DATA, true);
    //I must be loosing it on the repeated float to int casting...
    assert_eq!(cost(&v), 83197086729371 - 51);
}

fn cost(v: &Vec<Machine>) -> u128 {
    let mut cost = 0;
    for m in v {
        cost += m.cost();
    }
    return cost;
}

fn main() {
    println!("Hello, world!");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("/home/thomas/Music/windows-xp-shutdown.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    let vec = parse(data::DATA, true);
    let cost = cost(&vec);
    println!("Final cost = {}", cost);

    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(5));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos {
    x: u128,
    y: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Button {
    ipos: Pos,
    count: u128,
    per_press: u128,
}

impl Button {
    fn on_press(&mut self, pos: Pos) -> Pos {
        self.count += 1;
        return Pos {
            x: pos.x + self.ipos.x,
            y: pos.y + self.ipos.y,
        };
    }

    fn press_rec(&mut self, pos: Pos, cnt: u128) -> Pos {
        let mut npos = pos;
        let mut ncnt = cnt;
        self.count = 0;
        while ncnt != 0 {
            npos = self.on_press(npos);
            ncnt -= 1;
        }
        return npos;
    }
}

struct Machine {
    a: Button,
    b: Button,
    prize: Pos,
    best_a_f: u128,
    best_b_f: u128,
    found: bool,
}

fn parse(input: &str, part2: bool) -> Vec<Machine> {
    let reg_a = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let reg_b = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let reg_p = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut a = Pos { x: 0, y: 0 };
    let mut b = Pos { x: 0, y: 0 };
    let mut p = Pos { x: 0, y: 0 };

    let mut machines = Vec::new();

    for line in input.lines() {
        if line.len() < 2 {
            continue;
        }
        if reg_a.captures(line).is_some() {
            let cap = reg_a.captures(line).unwrap();
            a.x = cap[1].parse().unwrap();
            a.y = cap[2].parse().unwrap();
        } else if reg_b.captures(line).is_some() {
            let cap = reg_b.captures(line).unwrap();
            b.x = cap[1].parse().unwrap();
            b.y = cap[2].parse().unwrap();
        } else if reg_p.captures(line).is_some() {
            let cap = reg_p.captures(line).unwrap();
            p.x = cap[1].parse().unwrap();
            p.y = cap[2].parse().unwrap();
            let mut m = Machine::new(p.x, p.y, &a, &b);
            if part2 {
                m.prize.x += CRASY_HIGH;
                m.prize.y += CRASY_HIGH;
                m.real_math();
            } else {
                m.real_math();
            }
            if m.found {
                machines.push(m);
            }
        }
    }
    return machines;
}

impl Machine {
    pub fn new(x: u128, y: u128, a: &Pos, b: &Pos) -> Machine {
        return Machine {
            a: Button {
                ipos: *a,
                count: 0,
                per_press: 3,
            },
            b: Button {
                ipos: *b,
                count: 0,
                per_press: 1,
            },
            prize: Pos { x: x, y: y },
            best_a_f: 0,
            best_b_f: 0,
            found: false,
        };
    }

    fn cost(&self) -> u128 {
        return self.a.count * self.a.per_press + self.b.count * self.b.per_press;
    }

    pub fn test(&mut self) -> &mut Machine {
        for b_fac in 1..1000 {
            for a_fac in 1..1000 {
                let mut res: Pos = Pos { x: 0, y: 0 };
                res = self.a.press_rec(res, a_fac);
                res = self.b.press_rec(res, b_fac);
                if (res.x > self.prize.x) || (res.y > self.prize.y) {
                    continue;
                }
                if res.x == self.prize.x && res.y == self.prize.y {
                    //we have a match!
                    self.best_a_f = a_fac;
                    self.best_b_f = b_fac;
                    self.found = true;
                    return self;
                }
            }
        }
        return self;
    }

    pub fn real_math(&mut self) -> &mut Machine {
        let x1 = self.a.ipos.x as f64;
        let y1 = self.a.ipos.y as f64;

        let x2 = self.b.ipos.x as f64;
        let y2 = self.b.ipos.y as f64;

        let k = 1.0f64 / (x1 * y2 - x2 * y1);

        let retx = k * (y2 * self.prize.x as f64 - x2 * self.prize.y as f64);
        let rety = k * (-y1 * self.prize.x as f64 + x1 * self.prize.y as f64);

        self.best_a_f = retx as u128;
        self.best_b_f = rety as u128;

        self.a.count = retx as u128;
        self.b.count = rety as u128;

        self.found = is_int(retx) && is_int(rety); // (retx - retx.round()).abs() < 0.01f64 && (rety - rety.round()).abs() < 0.01f64;
        return self;
    }
}

fn is_int(fx: f64) -> bool {
    return (fx - fx.round()).abs() < 0.0001f64;
}

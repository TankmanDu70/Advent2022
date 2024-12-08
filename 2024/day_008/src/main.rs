use std::collections::HashMap;

mod data;

#[test]
fn test_008() {
    let mut map = Map::new();
    map.fill(data::TEST);
    assert_eq!(map.nodes.get(&'A').unwrap().len(), 3usize);
    assert_eq!(map.nodes.get(&'0').unwrap().len(), 4usize);
    assert_eq!(map.antinodes(), 14u32);
    assert_eq!(map.print(), data::EXPECTED);

    let mut map2: Map = Map::new();
    map2.fill(data::TEST);
    assert_eq!(map2.rep_antinodes(), 34u32);
    assert_eq!(map2.print(), data::EXPECTED2);
}

fn main() {
    println!("Hello, world!");
    explore(data::DATA);
    explore2(data::DATA);
}

fn explore(input: &str) {
    let mut map = Map::new();
    map.fill(input);
    println!("{}", map.antinodes());
    map.print();
}

fn explore2(input: &str) {
    let mut map = Map::new();
    map.fill(input);
    println!("{}", map.rep_antinodes());
    map.print();
}

struct Map {
    nodes: HashMap<char, Vec<Station>>,
    antinodes: Vec<Station>,
    xmax: usize,
    ymax: usize,
}

impl Map {
    pub fn new() -> Map {
        Map {
            nodes: HashMap::new(),
            antinodes: Vec::new(),
            xmax: 0usize,
            ymax: 0usize,
        }
    }
    pub fn fill(&mut self, data: &str) {
        for line in data.lines() {
            for x in 0..line.len() {
                self.xmax = line.len();
                let k = line.chars().nth(x).unwrap();
                if k != '.' {
                    if !self.nodes.contains_key(&k.clone()) {
                        self.nodes.insert(k, Vec::new());
                    }
                    self.nodes.get_mut(&k).unwrap().push(Station::new(
                        x.try_into().expect("cast!"),
                        self.ymax.try_into().expect("cast!"),
                    ));
                }
            }
            self.ymax += 1;
        }
    }

    pub fn antinodes(&mut self) -> u32 {
        let mut ret = 0u32;
        for v in &mut self.nodes.iter() {
            for station in v.1.iter() {
                for other in v.1.iter() {
                    if other.pos == station.pos {
                        continue;
                    }
                    let dist = station.pos.dist(&other.pos);
                    let antipos = station.pos.shift(&dist);
                    if self.fits(&antipos) {
                        let sta = Station::new(antipos.x, antipos.y);
                        if !self.antinodes.contains(&sta) {
                            self.antinodes.push(sta.clone());
                            for v in &mut self.nodes.iter() {
                                if v.1.contains(&sta) {
                                    break;
                                }
                            }
                            ret += 1;
                        }
                    }
                }
            }
        }
        return ret;
    }

    pub fn rep_antinodes(&mut self) -> u32 {
        let mut ret = 0u32;
        for v in &mut self.nodes.iter() {
            if v.1.len() < 3 {
                continue;
            }
            for station in v.1.iter() {
                ret += 1;
                for other in v.1.iter() {
                    if other.pos == station.pos {
                        continue;
                    }
                    let dist = station.pos.dist(&other.pos);
                    let mut antipos = station.pos.shift(&dist);
                    while self.fits(&antipos) {
                        let sta = Station::new(antipos.x, antipos.y);
                        if !self.antinodes.contains(&sta) {
                            self.antinodes.push(sta.clone());
                            for v in &mut self.nodes.iter() {
                                if v.1.contains(&sta) {
                                    ret -= 1;
                                    break;
                                }
                            }
                            ret += 1;
                        }
                        antipos = antipos.shift(&dist);
                    }
                }
            }
        }
        return ret;
    }

    pub fn fits(&self, pos: &Pos) -> bool {
        return pos.x < self.xmax.try_into().expect("cast!")
            && pos.y < self.ymax.try_into().expect("cast!")
            && pos.x >= 0
            && pos.y >= 0;
    }

    pub fn print(&self) -> String {
        let mut ret = "".to_string();
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                let mut written: bool = false;
                for n in self.nodes.iter() {
                    if n.1
                        .contains(&Station::new(x.try_into().unwrap(), y.try_into().unwrap()))
                    {
                        ret.push_str(&n.0.to_string());
                        written = true;
                        continue;
                    }
                }
                if self
                    .antinodes
                    .contains(&Station::new(x.try_into().unwrap(), y.try_into().unwrap()))
                    && !written
                {
                    ret.push_str(&'#'.to_string());
                } else if !written {
                    ret.push_str(&'.'.to_string());
                }
            }
            ret.push_str(&'\n'.to_string());
        }
        print!("{}", ret);
        return ret;
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Station {
    pos: Pos,
}

impl Station {
    pub fn new(x: isize, y: isize) -> Station {
        Station {
            pos: Pos::new(x, y),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos { x: x, y: y }
    }

    pub fn dist(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn shift(&self, dist: &Pos) -> Pos {
        Pos {
            x: self.x + dist.x,
            y: self.y + dist.y,
        }
    }
}

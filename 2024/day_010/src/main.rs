use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::usize;
use uuid::Uuid;

mod data;

#[test]
fn test_aoc() {
    let mut grp = Group::new(data::TEST1);

    assert_eq!(grp.map.starts.len(), 1);
    assert_eq!(grp.map.starts, vec![Pos { x: 3, y: 0 }]);
    grp.hike_until_death(false);
    assert_eq!(grp.scores(), 2);
    assert_eq!(
        grp.paths,
        HashMap::from([(
            Pos { x: 3, y: 0 },
            vec![Pos { x: 0, y: 6 }, Pos { x: 6, y: 6 }]
        )])
    );

    let mut grp = Group::new(data::TEST2);

    assert_eq!(grp.map.starts.len(), 2);
    grp.hike_until_death(false);
    assert_eq!(grp.scores(), 3);
    assert_eq!(
        grp.paths,
        HashMap::from([
            (Pos { x: 1, y: 0 }, vec![Pos { x: 3, y: 5 }]),
            (
                Pos { x: 5, y: 6 },
                vec![Pos { x: 3, y: 5 }, Pos { x: 4, y: 0 }]
            )
        ])
    );

    let mut grp = Group::new(data::TEST4);
    grp.hike_until_death(false);
    assert_eq!(grp.scores(), 4);

    let mut grp = Group::new(data::TEST);

    assert_eq!(grp.map.starts.len(), 9);
    assert_eq!(
        grp.map.starts,
        vec![
            Pos { x: 2, y: 0 },
            Pos { x: 4, y: 0 },
            Pos { x: 4, y: 2 },
            Pos { x: 6, y: 4 },
            Pos { x: 2, y: 5 },
            Pos { x: 5, y: 5 },
            Pos { x: 0, y: 6 },
            Pos { x: 6, y: 6 },
            Pos { x: 1, y: 7 }
        ]
    );
    grp.hike_until_death(false);
    assert_eq!(grp.scores(), 36);

    let map = Map::new(data::DATA);
    assert_eq!(map.starts.len(), 188);
}

fn main() {
    println!("Hello, world!");

    let mut grp = Group::new(data::DATA);
    grp.hike_until_death(false);
    grp.scores();

    let mut grp = Group::new(data::DATA);
    grp.hike_until_death(true);
    grp.scores();
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    DIR { x: isize, y: isize },
}

impl Dir {
    pub fn name(&self) -> &str {
        match self {
            Dir::DIR { x: -1, y: 0 } => "LEFT",
            Dir::DIR { x: 1, y: 0 } => "RIGHT",
            Dir::DIR { x: 0, y: -1 } => "UP",
            Dir::DIR { x: 0, y: 1 } => "DOWN",
            _ => "UNKNOWN",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hiker {
    id: usize,
    pos: Pos,
    origin: Pos,
    dir: Dir,
}
impl Hiker {
    pub fn new(id: usize, pos: Pos, origin: Pos, dir: Dir) -> Hiker {
        Hiker {
            id: id,
            pos: pos.clone(),
            origin: origin,
            dir: dir,
        }
    }
}

struct Group {
    hikers: Vec<Hiker>,
    map: Map,
    paths: HashMap<Pos, Vec<Pos>>,
}
impl Group {
    pub fn new(input: &str) -> Group {
        let map = Map::new(input);
        let mut hikers = Vec::new();

        for start in &map.starts {
            println!("new hiker at {:?}", start);
            hikers.push(Hiker::new(
                hikers.len(),
                start.clone(),
                start.clone(),
                Dir::DIR { x: -1, y: 0 },
            ));
        }
        Group {
            hikers: hikers,
            map: map,
            paths: HashMap::new(),
        }
    }
    pub fn kill_hiker(&mut self, i: usize) {
        println!(
            "Hiker {} killed at {:?}",
            self.hikers[i].id, self.hikers[i].pos
        );
        self.hikers.remove(i);
    }
    pub fn reached(&mut self, i: usize, dupp: bool) -> bool {
        let h = &mut self.hikers[i];
        let pos = h.pos.clone();
        let ret = self.map.cells[pos.y as usize][pos.x as usize] == 9;
        if ret {
            let orig = h.origin.clone();
            if !self.paths.contains_key(&orig) {
                println!("Hiker {} from {:?} arrived at {:?}", h.id, orig, pos);
                let vec = vec![pos];
                self.paths.insert(orig.clone(), vec);
            } else {
                if !self.paths.get(&orig).unwrap().contains(&pos) || dupp {
                    self.paths.get_mut(&orig).unwrap().push(pos);
                }
            }
        }
        return ret;
    }
    pub fn scores(&mut self) -> u32 {
        let mut ret = 0u32;
        for path in self.paths.iter() {
            println!("From {:?} we get {:?}", path.0, path.1);
            ret += path.1.len() as u32;
        }
        println!("Total score is {}", ret);
        return ret;
    }
    pub fn hikable(&mut self, target: &Pos, hk: usize) -> bool {
        let hiker = self.hikers[hk].pos.clone();
        if target.x < self.map.max.x && target.y < self.map.max.y && target.x >= 0 && target.y >= 0
        {
            return (self.map.cells[target.y as usize][target.x as usize]
                - self.map.cells[hiker.y as usize][hiker.x as usize])
                == 1;
        } else {
            return false;
        }
    }
    pub fn advance(&mut self, hk: usize) -> bool {
        let pos = self.hikers[hk].pos.clone();
        if self.map.lookup(&pos, &self.hikers[hk].dir).is_none() {
            return false;
        }
        let new_pos: Pos = self.map.lookup(&pos, &self.hikers[hk].dir).unwrap();
        if self.hikable(&new_pos, hk) {
            self.hikers[hk].pos = new_pos;
            println!(
                "Hiker {} is at {:?}",
                self.hikers[hk].id, self.hikers[hk].pos
            );
            return true;
        }
        return false;
    }
    pub fn hike_until_death(&mut self, dupp: bool) {
        for hk in 0..self.hikers.len() {
            self.count_choices(hk);
        }
        while self.hikers.len() > 0 {
            //println!("There are {} hikers !", self.hikers.len());
            for hk in 0..self.hikers.len() {
                if self.reached(hk, dupp) {
                    self.kill_hiker(hk);
                    break;
                } else {
                    let hiker = self.hikers[hk].clone();

                    if self.map.lookup(&hiker.pos, &hiker.dir).is_none() {
                        self.kill_hiker(hk);
                        break;
                    }
                    if self.advance(hk) {
                        self.count_choices(hk);
                        break;
                    } else {
                        self.kill_hiker(hk);
                        break;
                    }
                }
            }
        }
    }
    pub fn count_choices(&mut self, hk: usize) -> usize {
        let mut ret = 0;
        let mut options: Vec<(Dir, bool)> = vec![
            (Dir::DIR { x: -1, y: 0 }, false),
            (Dir::DIR { x: 1, y: 0 }, false),
            (Dir::DIR { x: 0, y: 1 }, false),
            (Dir::DIR { x: 0, y: -1 }, false),
        ];
        let clone: Hiker = self.hikers[hk].clone();

        let mut id = 0;
        for op in &mut options {
            id += 1;
            let target = self.map.lookup(&clone.pos.clone(), &op.0);
            if target.is_some() {
                if self.hikable(&target.unwrap(), hk) {
                    op.1 = true;
                }
            }
        }
        let mut reassigned = false;
        for c in 0..options.len() {
            if options[c].1 == true {
                if !reassigned {
                    self.hikers[hk].dir = options[c].0.clone();
                    reassigned = true;
                    println!(
                        "hiker {:?} going {:?} {:?}",
                        clone,
                        self.hikers[hk].id,
                        self.hikers[hk].dir.name()
                    );
                } else {
                    self.hikers.push(Hiker::new(
                        self.hikers.len(),
                        clone.pos.clone(),
                        clone.origin.clone(),
                        options[c].0.clone(),
                    ));
                    println!(
                        "hiker {:?} poped {:?} {:?}",
                        clone,
                        self.hikers.last().unwrap().id,
                        options[c].0.name()
                    );
                }
                ret += 1;
            }
        }
        //println!("total {} choices", ret);
        return ret;
    }
    pub fn has_choice(&mut self, hk: usize) -> usize {
        let mut choices = 0;
        let options: Vec<Dir> = vec![
            Dir::DIR { x: -1, y: 0 },
            Dir::DIR { x: 1, y: 0 },
            Dir::DIR { x: 0, y: 1 },
            Dir::DIR { x: 0, y: -1 },
        ];
        let clone: Hiker = self.hikers[hk].clone();
        for op in options {
            if self.map.lookup(&clone.pos.clone(), &op).is_some() {
                choices += 1;
            }
        }
        return choices;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos { x: x, y: y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    cells: Vec<Vec<i32>>,
    starts: Vec<Pos>,
    pos: Pos,
    max: Pos,
}

impl Map {
    pub fn new(input: &str) -> Map {
        let mut cells = Vec::new();
        let mut starts: Vec<Pos> = Vec::new();

        let mut x: isize = 0isize;
        let mut xmax = 0isize;
        let mut y = 0isize;

        for line in input.split("\n") {
            x = 0;
            let mut vec: Vec<i32> = Vec::new();
            for c in line.chars() {
                if c.is_digit(10) {
                    vec.push(
                        c.to_digit(10)
                            .unwrap()
                            .try_into()
                            .expect("cast to i32 failed"),
                    );
                } else {
                    vec.push(0);
                }

                if c == '0' {
                    starts.push(Pos::new(x, y));
                }
                x += 1;
                if xmax < x {
                    xmax = x;
                }
            }
            cells.push(vec);
            y += 1;
        }
        let max = Pos::new(xmax, y);
        Map {
            cells: cells,
            starts: starts,
            pos: Pos::new(0, 0),
            max: max,
        }
    }

    pub fn starts(&self) -> &Vec<Pos> {
        return &self.starts;
    }

    pub fn lookup(&mut self, at: &Pos, dir: &Dir) -> Option<Pos> {
        let mut ret: Pos = at.clone();
        match dir {
            Dir::DIR { x: -1, y: 0 } => {
                if at.x > 0 {
                    ret.x -= 1;
                } else {
                    return None;
                }
            }
            Dir::DIR { x: 1, y: 0 } => {
                if at.x < self.max.x {
                    ret.x += 1;
                } else {
                    return None;
                }
            }
            Dir::DIR { x: 0, y: 1 } => {
                if at.y < self.max.y {
                    ret.y += 1;
                } else {
                    return None;
                }
            }
            Dir::DIR { x: 0, y: -1 } => {
                if at.y > 0 {
                    ret.y -= 1;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
        return Some(ret);
    }
}

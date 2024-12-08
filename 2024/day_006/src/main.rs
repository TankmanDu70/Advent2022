mod data;
extern crate clearscreen;
extern crate num_derive;
extern crate num_traits;
#[allow(unused)]
use clearscreen::clear;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{thread::sleep, time::Duration};

pub const DELAY: u32 = 100_000_000u32;

#[test]
fn test_004() {
    assert_eq!(
        data::TEST,
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    );
    let mut map: MazeT = MazeT::new(data::TEST);
    let mut guard = GuardT::new(&map, String::from("main"));
    explore(&mut guard, &mut map);
    assert_eq!(guard.visited, 41);

    let mut map2: MazeT = MazeT::new(data::TEST);
    let mut guard2 = GuardT::new(&map2, String::from("main"));
    findloops(&mut guard2, &mut map2);

    let mut b = BumperT::new(0, 0);
    for _i in 0..1000 {
        b.mark_collision(DirT::Top);
    }
    assert_eq!(b.mark_collision(DirT::Top), true);
    assert_eq!(b.mark_collision(DirT::Left), false);

    assert!(map2.inducers.contains(&BumperT::new(1, 8)));
    assert!(map2.inducers.contains(&BumperT::new(3, 8)));
    assert!(map2.inducers.contains(&BumperT::new(7, 7)));
    assert!(map2.inducers.contains(&BumperT::new(6, 7)));
    assert!(map2.inducers.contains(&BumperT::new(7, 9)));
    assert!(map2.inducers.contains(&BumperT::new(3, 6)));
    assert_eq!(guard2.loop_chance, 6);
}

fn explore(g: &mut GuardT, m: &mut MazeT) {
    g.locate(&m);
    while !g.got_out() {
        if !g.advance(m) {
            let ind = g.get_the_bump_ind(m).unwrap();
            g.rotate();
            if ind == usize::MAX {
                continue;
            }
            m.bumpers[ind].mark_collision(g.pos.dir);
        }
        m.mark(g.pos);
    }
}

fn findloops(g: &mut GuardT, m: &mut MazeT) {
    g.locate(&m);
    while !g.got_out() {
        let p_ind: CoordT = g.pos.next(g.pos.dir);
        if g.bumper_in_corner(&m) && m.is_free(&p_ind) {
            let mut loops = false;
            let mut newg = g.clone();
            m.cells[p_ind.y][p_ind.x] = '#';
            m.bumpers.push(BumperT::new(p_ind.x, p_ind.y));
            newg.name = String::from("Bouncer");

            newg.rotate();
            while !newg.got_out() && !loops {
                if !newg.advance(m) {
                    let ind: usize = newg.get_the_bump_ind(m).unwrap();
                    if ind == usize::MAX {
                        loops = true;
                        //means the virtual guard got out
                        continue;
                    }
                    let bump: BumperT =
                        BumperT::new(g.pos.next(g.pos.dir).x, g.pos.next(g.pos.dir).y);

                    if m.bumpers[ind].mark_collision(newg.pos.dir) {
                        if !g.loop_inducers.contains(&bump) {
                            g.loop_chance += 1;
                            g.loop_inducers.push(bump);
                            m.inducers.push(g.loop_inducers.last().unwrap().clone());
                        }
                        loops = true;
                    }
                    newg.rotate();
                }
            }
            m.bumpers.pop();
            m.cells[p_ind.y][p_ind.x] = '.';
            m.reset_bumpers();
        }
        if !g.advance(m) {
            g.rotate();
        };
        m.mark(g.pos);
    }
    m.print(0, false);
}

fn main() {
    println!("Hello, world!");
    let mut map2: MazeT = MazeT::new(data::DATA);
    let mut guard2 = GuardT::new(&map2, String::from("main"));
    findloops(&mut guard2, &mut map2);

    assert!(map2.cells[guard2.start.y][guard2.start.x] != 'O');
    println!(
        "The guard visited {} cells, loop chances found {}(={})",
        guard2.visited,
        guard2.loop_chance,
        guard2.loop_inducers.len()
    );
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
enum DirT {
    Top = 0,
    Right,
    Bottom,
    Left,
    Cnt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoordT {
    x: usize,
    y: usize,
    xmax: usize,
    ymax: usize,
    dir: DirT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BumperT {
    pos: CoordT,
    from: Vec<u16>,
}

impl BumperT {
    pub fn new(x: usize, y: usize) -> BumperT {
        BumperT {
            pos: CoordT::new(x, y, 0, 0),
            from: vec![0; 4],
        }
    }
    pub fn mark_collision(&mut self, dir: DirT) -> bool {
        let ind = <usize as FromPrimitive>::from_usize(dir as usize).unwrap();
        if self.from[ind] > 20 {
            return true;
        }
        self.from[ind] += 1;
        return false;
    }
}

impl CoordT {
    pub fn new(x: usize, y: usize, xmax: usize, ymax: usize) -> CoordT {
        CoordT {
            x: x,
            y: y,
            xmax: xmax,
            ymax: ymax,
            dir: DirT::Top,
        }
    }
    pub fn next(&self, dir: DirT) -> CoordT {
        let mut new_pos = self.clone();
        match dir {
            DirT::Top => {
                if self.y != 0 {
                    new_pos.y -= 1
                }
            }
            DirT::Right => {
                if self.x != self.xmax {
                    new_pos.x += 1
                }
            }
            DirT::Bottom => {
                if self.y != self.ymax {
                    new_pos.y += 1
                }
            }
            DirT::Left => {
                if self.x != 0 {
                    new_pos.x -= 1
                }
            }
            DirT::Cnt => {
                // this should not happen
                assert!(false)
            }
        }
        return new_pos;
    }
    pub fn got_out(&self) -> bool {
        return (self.x == self.xmax && self.dir == DirT::Right)
            || (self.x == 0 && self.dir == DirT::Left)
            || (self.y == self.ymax && self.dir == DirT::Bottom)
            || (self.y == 0 && self.dir == DirT::Top);
    }
}

#[derive(Debug, Clone)]
struct GuardT {
    name: String,
    pos: CoordT,
    start: CoordT,
    visited: u32,
    loop_chance: u32,
    loop_inducers: Vec<BumperT>,
}
impl GuardT {
    pub fn new(maze: &MazeT, name: String) -> GuardT {
        GuardT {
            name: name,
            pos: CoordT::new(0, 0, maze.cells[0].len() - 1, maze.cells.len() - 1),
            start: CoordT::new(0, 0, maze.cells[0].len() - 1, maze.cells.len() - 1),
            visited: 1,
            loop_chance: 0,
            loop_inducers: Vec::new(),
        }
    }
    pub fn locate(&mut self, maze: &MazeT) {
        //println!("Searching guard");
        for y in 0..maze.max_y() {
            for x in 0..maze.max_x() {
                if maze.cells[y][x] == '^' {
                    self.pos.x = x;
                    self.pos.y = y;
                    self.start.x = x;
                    self.start.y = y;
                    println!("Located guard {} at ({},{})", self.name, x, y);
                    return;
                }
            }
        }
        assert!(false);
    }
    pub fn rotate(&mut self) {
        self.pos.dir = FromPrimitive::from_u8((self.pos.dir as u8 + 1) % DirT::Cnt as u8).unwrap();
    }
    pub fn advance(&mut self, map: &MazeT) -> bool {
        // net method has limit;
        let new_pos = self.pos.next(self.pos.dir);
        if map.get_cell(new_pos.x, new_pos.y) == '#' {
            return false;
        } else {
            self.pos = new_pos;
            if map.get_cell(new_pos.x, new_pos.y) == '.' {
                self.visited += 1;
            }
        }
        return true;
    }
    pub fn get_the_bump_ind(&mut self, map: &MazeT) -> Option<usize> {
        if self.pos.got_out() {
            return Some(usize::MAX);
        }
        let new_pos: CoordT = self.pos.next(self.pos.dir);
        assert_eq!(map.get_cell(new_pos.x, new_pos.y), '#');
        for b in 0..map.bumpers.len() {
            if map.bumpers[b].pos.x == new_pos.x && map.bumpers[b].pos.y == new_pos.y {
                return Some(b);
            }
        }
        return None;
    }
    pub fn bumper_in_corner(&mut self, map: &MazeT) -> bool {
        for b in &map.bumpers {
            if (self.pos.x < b.pos.x && self.pos.y == b.pos.y && self.pos.dir == DirT::Top)
                || (self.pos.x == b.pos.x && self.pos.y < b.pos.y && self.pos.dir == DirT::Right)
                || (self.pos.x > b.pos.x && self.pos.y == b.pos.y && self.pos.dir == DirT::Bottom)
                || (self.pos.x == b.pos.x && self.pos.y > b.pos.y && self.pos.dir == DirT::Left)
            {
                return true;
            }
        }
        return false;
    }
    pub fn got_out(&self) -> bool {
        return self.pos.got_out();
    }
    #[allow(dead_code)]
    pub fn print_loops(&self) -> String {
        let mut ret = String::from("");
        for line in 0..self.pos.ymax {
            for cell in 0..self.pos.xmax {
                ret.push_str(".");
                for b in &self.loop_inducers {
                    if b.pos.y == line && b.pos.x == cell {
                        ret.pop();
                        ret.push_str("O");
                    }
                }
            }
            ret.push_str("\n");
        }
        return ret;
    }
}

#[derive(Debug)]
struct MazeT {
    cells: Vec<Vec<char>>,
    bumpers: Vec<BumperT>,
    inducers: Vec<BumperT>,
}

impl MazeT {
    pub fn new(input: &str) -> MazeT {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in input.split('\n') {
            map.push(line.chars().collect());
        }
        let mut bmprs = Vec::new();
        let indrs = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == '#' {
                    bmprs.push(BumperT::new(x, y));
                }
            }
        }
        for _b in &bmprs {}
        MazeT {
            cells: map,
            bumpers: bmprs,
            inducers: indrs,
        }
    }
    #[allow(dead_code)]
    pub fn refill(&mut self, input: &str) {
        if !self.cells.is_empty() {
            self.cells.clear();
        }
        for line in input.split('\n') {
            self.cells.push(line.chars().collect());
        }
    }
    pub fn get_cell(&self, x: usize, y: usize) -> char {
        return self.cells[y][x];
    }
    pub fn mark(&mut self, pos: CoordT) {
        self.cells[pos.y][pos.x] = match pos.dir {
            DirT::Top => '|',
            DirT::Bottom => '|',
            DirT::Left => '-',
            DirT::Right => '-',
            DirT::Cnt => '~',
        };
    }
    #[allow(dead_code)]
    pub fn mark_loop(&mut self, x: usize, y: usize) {
        self.cells[y][x] = 'O';
    }
    #[allow(dead_code)]
    pub fn print(&self, delay: u32, loop_only: bool) -> String {
        if delay != 0 {
            clearscreen::clear().expect("failed to clear screen");
        }
        let mut ret = String::from("");
        for line in 0..self.max_y() {
            for cell in 0..self.max_x() {
                if loop_only {
                    ret.push('.');
                } else {
                    ret.push_str(&self.cells[line][cell].to_string());
                }
                for l in &self.inducers {
                    if l.pos.x == cell && l.pos.y == line {
                        ret.pop();
                        ret.push('O');
                    }
                }
            }
            ret.push_str("\n");
        }
        println!("{}", ret.clone());
        if delay != 0 {
            sleep(Duration::new(0, delay));
        }
        return ret;
    }
    pub fn max_x(&self) -> usize {
        return self.cells[0].len();
    }
    pub fn max_y(&self) -> usize {
        return self.cells.len();
    }
    #[allow(dead_code)]
    pub fn is_free(&self, pos: &CoordT) -> bool {
        return self.cells[pos.y][pos.x] != '#';
    }
    pub fn reset_bumpers(&mut self) {
        for b in &mut self.bumpers {
            b.from = vec![0; 4];
        }
    }
}

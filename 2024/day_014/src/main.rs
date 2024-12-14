mod data;
use regex;
use rodio::{source::Source, Decoder, OutputStream};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_aoc() {
    let mut map = Map::new(data::TEST);
    map.move_all_robots(100);
    assert_eq!(map.print(), data::END100S);
    assert_eq!(map.safety(), 12);

    let mut map = Map::new(data::DATA);
    assert_eq!(map.move_all_robots(100).safety(), 218295000);

    let mut map = Map::new(data::DATA);
    assert_eq!(map.find_easter_egg(10usize).seconds, 6870);
}

fn main() {
    println!("Hello, world!");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("/home/thomas/Music/windows-xp-shutdown.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    let mut map = Map::new(data::DATA);
    map.find_easter_egg(10);
    println!("{} \n{} at {}", map.print(), map.safety(), map.seconds);
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(5));
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn wraparound(&mut self, max: Pos) -> &mut Pos {
        if self.x < 0 {
            self.x = (max.x - (self.x.abs() % (max.x + 1)) + 1) % (max.x + 1);
        } else {
            self.x = self.x % (max.x + 1);
        }
        if self.y < 0 {
            self.y = (max.y - (self.y.abs() % (max.y + 1)) + 1) % (max.y + 1);
        } else {
            self.y = self.y % (max.y + 1);
        }
        assert_ne!(self.x, max.x + 1);
        assert_ne!(self.y, max.y + 1);
        return self;
    }
}

struct Map {
    seconds: usize,
    max: Pos,
    robots: Vec<Robot>,
    grid: HashMap<Pos, isize>,
}

impl Map {
    fn find_easter_egg(&mut self, period: usize) -> &mut Map {
        self.max.x = 100;
        self.max.y = 102;

        self.seconds = 0;

        while !self.detect_tree() {
            self.seconds += period;
            self.move_all_robots(period);
        }
        return self;
    }

    fn new(data: &str) -> Map {
        let regex = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut robots = Vec::new();
        let mut grid = HashMap::<Pos, isize>::new();
        let mut max = Pos { x: 0, y: 0 };
        for line in data.lines() {
            let cap = regex.captures(line).unwrap();
            let new_r = Robot {
                pos: Pos {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                },
                spe: Pos {
                    x: cap[3].parse().unwrap(),
                    y: cap[4].parse().unwrap(),
                },
            };
            if max.x < new_r.pos.x {
                max.x = new_r.pos.x
            }
            if max.y < new_r.pos.y {
                max.y = new_r.pos.y
            }
            if grid.contains_key(&new_r.pos) {
                *grid.get_mut(&new_r.pos).unwrap() += 1;
            } else {
                grid.insert(new_r.pos, 1);
            }
            robots.push(new_r);
        }
        return Map {
            robots: robots,
            max: max,
            grid: grid,
            seconds: 0,
        };
    }
    fn move_robot(&mut self, id: usize, cnt: usize) -> &mut Map {
        let robot = &mut self.robots[id];
        update_grid(&mut self.grid, robot.pos, -1);
        robot.pos.x += cnt as isize * robot.spe.x;
        robot.pos.y += cnt as isize * robot.spe.y;
        robot.pos.wraparound(self.max);
        update_grid(&mut self.grid, robot.pos, 1);
        return self;
    }
    fn move_all_robots(&mut self, cnt: usize) -> &mut Map {
        for id in 0..self.robots.len() {
            self.move_robot(id, cnt);
        }
        return self;
    }
    fn print(&self) -> String {
        let mut ret = String::from("");
        for yy in 0..self.max.y as usize + 1 {
            for xx in 0..self.max.x as usize + 1 {
                let pos = Pos {
                    x: xx as isize,
                    y: yy as isize,
                };
                if self.grid.contains_key(&pos) {
                    let cnt = self.grid.get(&pos).unwrap();
                    if *cnt != 0 {
                        ret.push_str(cnt.to_string().as_str());
                    } else {
                        ret.push('.');
                    }
                } else {
                    ret.push('.');
                }
            }
            if yy != self.max.y as usize {
                ret.push('\n');
            }
        }
        return ret;
    }
    fn safety(&mut self) -> u32 {
        let mut result = vec![0; 4];
        let quad1 = vec![
            Pos { x: 0, y: 0 },
            Pos {
                x: self.max.x / 2 - 1,
                y: self.max.y / 2 - 1,
            },
        ];
        let quad2 = vec![
            Pos {
                x: self.max.x / 2 + 1,
                y: 0,
            },
            Pos {
                x: self.max.x,
                y: self.max.y / 2 - 1,
            },
        ];
        let quad3 = vec![
            Pos {
                x: 0,
                y: self.max.y / 2 + 1,
            },
            Pos {
                x: self.max.x / 2 - 1,
                y: self.max.y,
            },
        ];
        let quad4 = vec![
            Pos {
                x: self.max.x / 2 + 1,
                y: self.max.y / 2 + 1,
            },
            Pos {
                x: self.max.x,
                y: self.max.y,
            },
        ];

        let quads = vec![quad1, quad2, quad3, quad4];
        for ro in &self.robots {
            for qa in 0..quads.len() {
                if ro.pos.x >= quads[qa][0].x
                    && ro.pos.x <= quads[qa][1].x
                    && ro.pos.y >= quads[qa][0].y
                    && ro.pos.y <= quads[qa][1].y
                {
                    result[qa] += 1;
                }
            }
        }
        return result.iter().product();
    }

    fn detect_tree(&mut self) -> bool {
        let mut ret = 0;
        let mut per_ys: Vec<usize> = vec![0; self.max.y as usize + 1];
        let mut per_xs: Vec<usize> = vec![0; self.max.x as usize + 1];
        for rob in self.robots.iter() {
            per_ys[rob.pos.y as usize] += 1;
            per_xs[rob.pos.x as usize] += 1;
        }
        for y in per_ys {
            if y > 25 {
                ret += y;
                if ret > 100 {
                    return true;
                }
            }
        }
        for x in per_xs {
            if x > 25 {
                ret += x;
            }
            if ret > 100 {
                return true;
            }
        }
        return false;
    }
}

fn update_grid(grid: &mut HashMap<Pos, isize>, pos: Pos, cnt: isize) {
    if grid.contains_key(&pos) {
        *grid.get_mut(&pos).unwrap() += cnt;
    } else {
        grid.insert(pos, cnt);
    }
    assert!(*grid.get_mut(&pos).unwrap() >= 0);
}
struct Robot {
    pos: Pos,
    spe: Pos,
}

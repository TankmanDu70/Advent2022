use std::{collections::HashMap, hash::Hash, usize};

mod data;

#[test]
fn test_aoc() {
    let mut grid = Map::new(data::EX1);
    assert_eq!(grid.all_moves(true).best_score(), 7036);
    grid.all_moves(true).get_all_sits().printx();
    assert_eq!(grid.sits_count(), 45);

    let mut grid = Map::new(data::EX2);
    grid.all_moves(true).get_all_sits().printx();
    assert_eq!(grid.sits_count(), 64);

    let mut grid = Map::new(data::DATA);
    assert_eq!(grid.all_moves(true).best_score(), 114476);
    grid.all_moves(true).get_all_sits();
    assert!(grid.sits_count() == 508);
}

fn main() {
    println!("Hello, world!");
    let mut grid = Map::new(data::DATA);
    println!("Shortest path: {}", grid.all_moves(true).best_score());
    println!("Sit count: {}", grid.get_all_sits().sits_count());
    grid.printx();
}
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct Cell {
    pos: Pos,
    dir: char,
    counted: bool,
}
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Deer {
    id: usize,
    pos: Pos,
    rection: char, //(^-^)
    cost: usize,
    running: bool,
    cell_counter: usize,
    visited: Vec<Pos>,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct Sit {
    pos: Pos,
    counted: bool,
}

struct Map {
    sit_cnt: usize,
    running_deers: Vec<Deer>,
    highest_id: usize,
    goal: Pos,
    start: Pos,
    cells: Vec<Vec<char>>,
    paths: HashMap<Cell, usize>,
    sits: Vec<Cell>,
}

impl Map {
    fn new(input: &str) -> Map {
        let mut deer = Deer {
            id: 0,
            pos: Pos { x: 0, y: 0 },
            rection: '>',
            cost: 0,
            running: true,
            cell_counter: 0,
            visited: Vec::new(),
        };
        let mut _dv: Vec<Deer> = Vec::new();
        let mut goal = Pos { x: 0, y: 0 };
        let mut start = Pos { x: 0, y: 0 };
        let mut _x = 0usize;
        let mut _y = 0usize;
        let mut _bc = 0usize;
        let mut _vg: Vec<Vec<char>> = Vec::new();
        let mut _cmds = String::from("");

        for line in input.split('\n') {
            let mut _vl = Vec::new();
            _x = 0;

            if line.contains('<') || line.contains('>') || line.contains('v') || line.contains('^')
            {
                _cmds.push_str(line);
                continue;
            }

            for _c in line.chars() {
                if _c == 'S' {
                    start = Pos {
                        x: _x as isize,
                        y: _y as isize,
                    };
                    deer.pos.x = _x as isize;
                    deer.pos.y = _y as isize;
                    _dv.push(deer.clone());
                } else if _c == 'E' {
                    goal.x = _x as isize;
                    goal.y = _y as isize;
                } else if _c == '#' {
                    _bc += 1;
                }
                _vl.push(_c);
                _x += 1;
            }
            _vl.push('\n');
            _vg.push(_vl);
            _y += 1;
        }
        return Map {
            sit_cnt: 0,
            start: start,
            sits: Vec::new(),
            highest_id: 0,
            running_deers: _dv,
            cells: _vg,
            goal: goal,
            paths: HashMap::<Cell, usize>::new(),
        };
    }

    fn all_moves(&mut self, rot: bool) -> &mut Map {
        self.paths.insert(
            Cell {
                pos: self.start,
                dir: 'v',
                counted: false,
            },
            0,
        );
        while self.running_deers.len() > 0 {
            for ind in 0..self.running_deers.len() {
                if self.running_deers[ind].running {
                    self.explore(ind, rot);
                }
            }
            self.running_deers.retain(|deer| deer.running == true);
        }
        return self;
    }

    fn get_all_sits(&mut self) -> &mut Map {
        let _k: Cell = self.best_cell(&self.goal.clone());
        self.sits.push(_k);
        let mut left = true;
        while left {
            left = self
                .sits
                .iter()
                .position(|pos| pos.pos == self.start)
                .is_none()
                && self.sits.len() != 0;
            for ii in 0..self.sits.len() {
                let o_cell = self.sits[ii];
                if o_cell.counted {
                    left = false;
                    continue;
                }
                left = true;
                let mut match_cnt = 0;
                let mut all_pos = vec![o_cell.clone(); 4];
                all_pos[0].pos.x += 1;
                all_pos[1].pos.x -= 1;
                all_pos[2].pos.y += 1;
                all_pos[3].pos.y -= 1;

                let all_dir = vec!['^', '>', 'v', '<'];
                let mut found: bool = false;
                for dir_id in 0..all_dir.len() {
                    if found {
                        break;
                    }
                    for mut n_pos in all_pos.clone() {
                        n_pos.dir = all_dir[dir_id];
                        if self.paths.contains_key(&o_cell) {
                            let o_cost: usize = *self.paths.get(&o_cell).unwrap();
                            if self.paths.get(&n_pos).is_some() {
                                let n_cost = *self.paths.get(&n_pos).unwrap();
                                let mo_cost = o_cost % 1000;
                                if o_cost != 0 && mo_cost != 0 {
                                    if (o_cost == n_cost + 1 || o_cost == n_cost + 1001 )
                                        && o_cost <= self.best_score()
                                    {
                                        if self.sits.iter().position(|pos| *pos == n_pos).is_none()
                                        {
                                            self.sits.push(n_pos);
                                            match_cnt += 1;
                                            found = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if match_cnt == 0 {
                        self.sit_cnt += 1;
                        self.sits[ii].counted = true;
                    }
                }
            }
            //self.sits.retain(|sit| sit.counted != true);
        }
        return self;
    }

    /**
     * Starts from a cell and sends a deer every direction if passable;
     */
    fn explore(&mut self, ind: usize, rcost: bool) -> &mut Map {
        let all_dir = vec!['^', '>', 'v', '<'];
        let deer: Deer = self.running_deers[ind].clone();
        let mut dclone;
        for dir_id in 0..all_dir.len() {
            let c_dir = all_dir[dir_id];
            if c_dir != deer.rection {
                dclone = deer.clone();
                dclone.id = self.highest_id;
                self.highest_id += 1;
                let mut _dir = deer.rection;
                let mut _ind = all_dir
                    .iter()
                    .position(|_c| *_c == dclone.rection)
                    .unwrap()
                    .clone();
                let mut _rot = 0;
                while _dir != c_dir {
                    _ind = (_ind + 1) % all_dir.len();
                    _dir = all_dir[_ind];
                    _rot += 1;
                }
                if _rot == 2 {
                    continue;
                }
                dclone.rection = c_dir;
                if rcost {
                    dclone.cost += (_rot % 2) * 1000;
                    //*self.paths.get_mut(&dclone.pos).unwrap() = dclone.cost
                }
            } else {
                dclone = deer.clone();
            }
            dclone.pos = self.try_move(&dclone);

            if deer.pos != dclone.pos {
                dclone.cost += 1;
                let key = Cell {
                    pos: dclone.pos,
                    dir: dclone.rection,
                    counted: false,
                };
                if self.paths.contains_key(&key) {
                    dclone.running = !self.should_stop(&dclone);
                    if dclone.running {
                        *self.paths.get_mut(&key).unwrap() = dclone.cost;
                    }
                } else {
                    self.paths.insert(key, dclone.cost);
                    self.sit_cnt += 1;
                }
            } else {
                dclone.running = false;
            }

            if c_dir != deer.rection && dclone.running {
                self.running_deers.push(dclone);
            } else if c_dir == deer.rection {
                self.running_deers[ind] = dclone;
            }
        }
        return self;
    }

    /**
     * Returns the next position based on deer.rection;
     */
    fn try_move(&mut self, deer: &Deer) -> Pos {
        let mut n_pos = deer.pos.clone();
        match deer.rection {
            '<' => n_pos.x -= 1,
            '>' => n_pos.x += 1,
            'v' => n_pos.y += 1,
            '^' => n_pos.y -= 1,
            _ => println!("Unexpected dir : {}", deer.rection),
        }

        assert!((n_pos.y as usize) < self.cells.len());
        assert!((n_pos.x as usize) < self.cells[n_pos.y as usize].len());

        let _c: char = self.cells[n_pos.y as usize][n_pos.x as usize];
        if _c == '#' {
            //if blocked, returns same position.
            return deer.pos.clone();
        } else {
            return n_pos;
        }
    }

    /**
     *  Deer took a longer path than existing.
     */
    fn should_stop(&mut self, deer: &Deer) -> bool {
        let key = Cell {
            pos: deer.pos,
            dir: deer.rection,
            counted: false,
        };
        if self.paths.contains_key(&key) {
            if deer.cost >= *self.paths.get(&key).unwrap() {
                if deer.cost >= *self.paths.get(&key).unwrap() {
                    println!("{:?} MERGED", deer.pos);
                }
                return true;
            }
            return false;
        }
        if self.cells[deer.pos.y as usize][deer.pos.x as usize] == 'E' {
            println!("{} GOT IT", deer.id);
            self.sit_cnt += deer.cell_counter;
            return true;
        }
        return false;
    }

    /**
     *  Deer took a longer path than existing.
     */
    fn is_not_sit(&mut self, deer: &Deer) -> bool {
        let key = Cell {
            pos: deer.pos,
            dir: deer.rection,
            counted: false,
        };
        if self.paths.contains_key(&key) {
            let other_cost = *self.paths.get(&key).unwrap();
            if deer.cost == other_cost || deer.cost == (other_cost + 1000) {
                return false;
            }
            return true;
        }
        if self.cells[deer.pos.y as usize][deer.pos.x as usize] == 'S' {
            return true;
        }
        return false;
    }

    fn print(&mut self) -> String {
        let mut string = String::from("");
        let mut _y: isize = 0;
        for _line in &self.cells {
            let mut _x = 0;
            for _c in _line {
                if self
                    .sits
                    .iter()
                    .position(|sit| sit.pos == Pos { x: _x, y: _y })
                    .is_some()
                {
                    string.push('O');
                } else {
                    string.push(*_c);
                }
                _x += 1;
            }
            _y += 1;
        }
        return string;
    }

    fn printf(&mut self, all: bool) -> String {
        let mut string = String::from("");
        let mut _y: isize = 0;
        for _line in &mut self.cells {
            let mut _x = 0;
            let mut key = Cell {
                pos: Pos { x: 0, y: 0 },
                dir: '>',
                counted: false,
            };
            for _c in _line {
                let mut lowest = usize::MAX;
                let mut is_wall = true;
                for _dir in vec!['v', '<', '>', '^'] {
                    key = Cell {
                        pos: Pos { x: _x, y: _y },
                        dir: _dir,
                        counted: false,
                    };
                    if self.paths.contains_key(&key) {
                        let mut cost = usize::MAX;
                        is_wall = false;
                        for _path in &self.paths {
                            if *_path.1 < cost && _path.0.pos == key.pos {
                                key.dir = _path.0.dir;
                                cost = *_path.1;
                            }
                        }
                        let _val = self.paths.get(&key).unwrap();
                        if lowest > *_val {
                            lowest = *_val;
                        }
                    }
                }
                if !is_wall {
                    let mut name: String = String::from("........");
                    let mut val = 0;
                    for _dir in vec!['v', '<', '>', '^'] {
                        key.dir = _dir;
                        if self.paths.get(&key).is_some() {
                            if val > *self.paths.get(&key).unwrap() {
                                val = *self.paths.get(&key).unwrap();
                            }
                        }
                    }
                    name = val.to_string();
                    if self
                        .sits
                        .iter()
                        .position(|sit| sit.pos == Pos { x: _x, y: _y })
                        .is_some()
                        || all
                    {
                        string.push_str(name.as_str());
                        print!("[{:06}]", lowest.to_string());
                    } else {
                        print!("........");
                    }
                } else {
                    string.push_str("########");
                    print!("########");
                }
                _x += 1;
            }
            println!("");
            _y += 1;
        }
        return string;
    }

    fn printx(&mut self) -> String {
        let mut string = String::from("");
        let mut _y: isize = 0;
        for _line in &mut self.cells {
            let mut _x = 0;
            let mut key = Cell {
                pos: Pos { x: 0, y: 0 },
                dir: '>',
                counted: false,
            };
            for _c in _line {
                if *_c == '\n' {
                    continue;
                }
                let mut is_wall = true;
                for _dir in vec!['v', '<', '>', '^'] {
                    key = Cell {
                        pos: Pos { x: _x, y: _y },
                        dir: _dir,
                        counted: false,
                    };
                    if self.paths.contains_key(&key) {
                        let mut cost = usize::MAX;
                        is_wall = false;
                        for _path in &self.paths {
                            if *_path.1 < cost && _path.0.pos == key.pos {
                                key.dir = _path.0.dir;
                                cost = *_path.1;
                            }
                        }
                    }
                }
                if !is_wall {
                    let mut name: String = String::from(".");
                    let mut val = 0;
                    for _dir in vec!['v', '<', '>', '^'] {
                        key.dir = _dir;
                        if self.paths.get(&key).is_some() {
                            if val > *self.paths.get(&key).unwrap() {
                                val = *self.paths.get(&key).unwrap();
                            }
                        }
                    }
                    name = val.to_string();
                    if self
                        .sits
                        .iter()
                        .position(|sit| sit.pos == Pos { x: _x, y: _y })
                        .is_some()
                    {
                        string.push_str(name.as_str());
                        print!("O");
                    } else {
                        print!(".");
                    }
                } else {
                    string.push_str("#");
                    print!("#");
                }
                _x += 1;
            }
            println!("");
            _y += 1;
        }
        return string;
    }

    fn best_score(&mut self) -> usize {
        let mut lowest = usize::MAX;
        for _dir in vec!['v', '<', '>', '^'] {
            let key = Cell {
                pos: Pos {
                    x: self.goal.x,
                    y: self.goal.y,
                },
                dir: _dir,
                counted: false,
            };
            if self.paths.contains_key(&key) {
                let _val = self.paths.get(&key).unwrap();
                if lowest > *_val {
                    lowest = *_val;
                }
            }
        }
        return lowest;
    }

    fn best_cell(&mut self, pos: &Pos) -> Cell {
        let mut cell = Cell {
            pos: *pos,
            dir: '>',
            counted: false,
        };
        let mut cost = usize::MAX;
        for _path in &self.paths {
            if *_path.1 < cost && _path.0.pos == cell.pos {
                cell.dir = _path.0.dir;
                cost = *_path.1;
            }
        }
        return cell;
    }

    fn sits_count(&mut self) -> usize {
        let mut sits = Vec::new();
        sits.push(self.sits[0].pos);
        for _sit in 0..self.sits.len() {
            let cell = self.sits[_sit].pos;
            if sits.iter().position(|xx| *xx == cell).is_none() {
                sits.push(cell);
            }
        }
        return sits.len();
    }
}

mod data;

static mut rec_cnt: u32 = 0;
#[test]
fn test_aoc() {
    let mut grid = Map::new(data::EXMPL, false);
    assert_eq!(grid.print(), data::MV0);
    assert_eq!(unsafe { grid.all_moves().print() }, data::MV15);
    assert_eq!(grid.coordinates_sum(), 2028);

    let mut grid = Map::new(data::TEST, false);
    assert_eq!(unsafe { grid.all_moves().print() }, data::TESTR);
    assert_eq!(grid.coordinates_sum(), 10092);

    let mut grid = Map::new(data::DATA, false);
    assert_eq!(unsafe { grid.all_moves() }.coordinates_sum(), 1486930);

    let mut grid = Map::new(data::TEST, true);
    assert_eq!(unsafe { grid.all_moves() }.print(), data::TESTR2R);
    assert_eq!(grid.coordinates_sum_2(), 9021);
 
    let mut grid = Map::new(data::DATA, true);
    assert_eq!(unsafe { grid.all_moves() }.coordinates_sum(), 1492011);
}

fn main() {
    println!("Hello, world!");
    let mut grid = Map::new(data::DATA, false);
    println!(
        "Cordinates part 1: {}",
        unsafe { grid.all_moves() }.coordinates_sum()
    );

    let mut grid = Map::new(data::DATA, true);
    println!(
        "Cordinates part 2: {}",
        unsafe { grid.all_moves() }.coordinates_sum_2()
    );
    println!("{}", grid.print());
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Box {
    pos: Vec<Pos>,
}
struct Map {
    robot: Pos,
    box_cnt: usize,
    boxes: Vec<Box>,
    walls: Vec<Pos>,
    cells: Vec<Vec<char>>,
    commands: String,
    part2: bool,
}

impl Map {
    fn new(input: &str, part2: bool) -> Map {
        let mut robot = Pos { x: 0, y: 0 };
        let mut _x = 0usize;
        let mut _y = 0usize;
        let mut _bc = 0usize;
        let mut _walls = Vec::new();
        let mut _boxes = Vec::new();
        let mut _vg: Vec<Vec<char>> = Vec::new();
        let mut _cmds = String::from("");

        for line in input.lines() {
            let mut _vl = Vec::new();
            _x = 0;

            if line.contains('<') || line.contains('>') || line.contains('v') || line.contains('^')
            {
                _cmds.push_str(line);
                continue;
            }

            for _c in line.chars() {
                if _c == '@' {
                    robot.x = _x as isize;
                    robot.y = _y as isize;
                } else if _c == '#' {
                } else if _c == 'O' {
                    _bc += 1;
                } else if _c != '.' {
                    continue;
                }
                if part2 {
                    if _c == '@' {
                        _vl.push('@');
                        _vl.push('.');
                        _x += 1;
                    } else if _c == 'O' {
                        _vl.push('[');
                        _vl.push(']');
                        _x += 1;
                        _boxes.push(Box {
                            pos: vec![
                                Pos {
                                    x: _x as isize - 1,
                                    y: _y as isize,
                                },
                                Pos {
                                    x: _x as isize,
                                    y: _y as isize,
                                },
                            ],
                        });
                    } else {
                        _vl.push(_c);
                        _vl.push(_c);
                        _x += 1;
                    }
                } else {
                    _vl.push(_c);
                }
                _x += 1;
            }
            _vl.push('\n');
            _vg.push(_vl);
            _y += 1;
        }
        return Map {
            robot: robot,
            cells: _vg,
            commands: _cmds,
            box_cnt: _bc,
            part2: part2,
            boxes: _boxes,
            walls: _walls,
        };
    }

    unsafe fn all_moves(&mut self) -> &mut Map {
        let commands = self.commands.clone();
        // Interresting to find a smarter way here
        for _c in commands.chars() {
            self.move_bot(_c);
        }
        return self;
    }

    unsafe fn move_bot(&mut self, cmd: char) -> &mut Map {
        let r_pos = self.robot.clone();
        let mut _n_r_pos = r_pos.clone();
        if self.part2 {
            if self.peek_move(&r_pos, cmd) {
                _n_r_pos = self.try_move_2(&r_pos, cmd);
            }
        } else {
            _n_r_pos = self.try_move(&r_pos, cmd);
        }
        self.robot = _n_r_pos;
        return self;
    }

    unsafe fn try_move_2(&mut self, pos: &Pos, dir: char) -> Pos {
        let mut n_pos = pos.clone();
        let mut _current_bx_id = 0;
        let mut _bx_in_way_id = 0;
        let mut _is_box = false;

        for id in 0..self.boxes.len() {
            if self.boxes[id].pos.contains(&pos) {
                _current_bx_id = id;
                _is_box = true;
                //Finds wether starting point belongs to a box
                break;
            }
        }

        rec_cnt += 1;
        match dir {
            '<' => n_pos.x -= 1,
            '>' => n_pos.x += 1,
            'v' => n_pos.y += 1,
            '^' => n_pos.y -= 1,
            _ => println!("Unexpected dir : {}", dir),
        }

        assert!((n_pos.y as usize) < self.cells.len());
        assert!((n_pos.x as usize) < self.cells[n_pos.y as usize].len());

        for id in 0..self.boxes.len() {
            if self.boxes[id].pos.contains(&n_pos) {
                if _is_box && _current_bx_id == id {
                    //from the same box! We allow the move as it is a solid
                    rec_cnt -= 1;
                    return n_pos;
                }
                // must check both members are movable
                let mut move_cnt = 0;
                //position if move successfull
                let mut _n_box_pos = self.boxes[id].pos.clone();
                //position before move
                let _p_box_pos = self.boxes[id].pos.clone();
                for _box_pos in self.boxes[id].pos.clone() {
                    //Trying a move with both coordinates of the box;
                    let other_pos = _box_pos.clone();
                    if self.peek_move(&other_pos, dir) {
                        let other_n_pos = self.try_move_2(&other_pos, dir);
                        if other_n_pos == other_pos {
                            break;
                        } else {
                            _n_box_pos[move_cnt] = other_n_pos;
                            move_cnt += 1;
                            continue;
                        }
                    }
                }
                if move_cnt == 2 {
                    //Both members of the box were movable, updating grid!
                    rec_cnt -= 1;
                    for bxid in 0..self.boxes.len() {
                        if id != bxid {
                            //collision!
                            assert_ne!(self.boxes[bxid].pos, _n_box_pos);
                        }
                    }
                    self.boxes[id].pos = _n_box_pos;
                    return n_pos;
                }
            }
        }

        let _c: char = self.cells[n_pos.y as usize][n_pos.x as usize];
        if _c == '#' {
            rec_cnt -= 1;
            return pos.clone();
        }
        return n_pos;
    }

    unsafe fn peek_move(&mut self, pos: &Pos, dir: char) -> bool {
        let mut n_pos = pos.clone();
        let mut _current_bx_id = 0;
        let mut _bx_in_way_id = 0;
        let mut _is_box = false;

        for id in 0..self.boxes.len() {
            if self.boxes[id].pos.contains(&pos) {
                _current_bx_id = id;
                _is_box = true;
                //Finds wether starting point belongs to a box
                break;
            }
        }

        rec_cnt += 1;
        match dir {
            '<' => n_pos.x -= 1,
            '>' => n_pos.x += 1,
            'v' => n_pos.y += 1,
            '^' => n_pos.y -= 1,
            _ => println!("Unexpected dir : {}", dir),
        }

        assert!((n_pos.y as usize) < self.cells.len());
        assert!((n_pos.x as usize) < self.cells[n_pos.y as usize].len());

        for id in 0..self.boxes.len() {
            if self.boxes[id].pos.contains(&n_pos) {
                if _is_box && _current_bx_id == id {
                    //from the same box! We allow the move as it is a solid
                    rec_cnt -= 1;
                    return true;
                }
            }
        }

        for id in 0..self.boxes.len() {
            if self.boxes[id].pos.contains(&n_pos) {
                // must check both members are movable
                let mut move_cnt = 0;
                //position if move successfull
                let mut _n_box_pos = self.boxes[id].pos.clone();
                //position before move
                let _p_box_pos = self.boxes[id].pos.clone();
                for _box_pos in self.boxes[id].pos.clone() {
                    //Trying a move with both coordinates of the box;
                    let other_pos = _box_pos.clone();
                    if self.peek_move(&other_pos, dir) {
                        move_cnt += 1;
                    } else {
                        break;
                    }
                }
                if move_cnt != 2 {
                    //Both members of the box were movable !
                    rec_cnt -= 1;
                    //not so fast, one box doenst mean all boxes!
                    return false;
                }
            }
        }

        let _c: char = self.cells[n_pos.y as usize][n_pos.x as usize];
        if _c == '#' {
            rec_cnt -= 1;
            return false;
        }
        rec_cnt -= 1;
        return true;
    }

    fn try_move(&mut self, pos: &Pos, dir: char) -> Pos {
        let mut n_pos = pos.clone();
        match dir {
            '<' => n_pos.x -= 1,
            '>' => n_pos.x += 1,
            'v' => n_pos.y += 1,
            '^' => n_pos.y -= 1,
            _ => println!("Unexpected dir : {}", dir),
        }

        assert!((n_pos.y as usize) < self.cells.len());
        assert!((n_pos.x as usize) < self.cells[n_pos.y as usize].len());

        let _c: char = self.cells[n_pos.y as usize][n_pos.x as usize];
        if _c == '#' {
            return pos.clone();
        } else if _c == 'O' {
            let other_pos = n_pos.clone();
            let other_n_pos = self.try_move(&other_pos, dir);
            if other_n_pos != other_pos {
                let prev = self.cells[pos.y as usize][pos.x as usize];
                self.cells[n_pos.y as usize][n_pos.x as usize] = prev;
                self.cells[pos.y as usize][pos.x as usize] = '.';
                return n_pos;
            }
        } else if _c == '.' {
            let prev = self.cells[pos.y as usize][pos.x as usize];
            self.cells[n_pos.y as usize][n_pos.x as usize] = prev;
            self.cells[pos.y as usize][pos.x as usize] = '.';
            return n_pos;
        }
        return pos.clone();
    }

    fn print(&mut self) -> String {
        let mut string = String::from("");
        if self.part2 {
            for _y in 0..self.cells.len() {
                for _x in 0..self.cells[_y].len() {
                    if vec!['[', ']', '@'].contains(&self.cells[_y][_x]) {
                        self.cells[_y][_x] = '.';
                    }
                    if self.robot.x == _x as isize && self.robot.y == _y as isize {
                        self.cells[_y][_x] = '@';
                    }
                }
            }
            for _box in &mut self.boxes {
                for _memb in 0.._box.pos.len() {
                    let _x = _box.pos[_memb].x as usize;
                    let _y = _box.pos[_memb].y as usize;
                    // box likely fragmented
                    //assert_ne!(vec!['[', ']'].contains(&self.cells[_y][_x]), true);
                    if _memb == 0 {
                        self.cells[_y][_x] = '[';
                    } else {
                        self.cells[_y][_x] = ']';
                    }
                }
            }
        }
        for _line in &self.cells {
            let _s: String = _line.into_iter().collect();
            string.push_str(_s.as_str());
        }
        return string;
    }

    fn coordinates_sum(&self) -> u32 {
        let mut bc: usize = self.box_cnt;
        let mut cost = 0u32;
        for _y in 0..self.cells.len() {
            for _x in 0..self.cells[_y].len() {
                if self.cells[_y][_x] == 'O' {
                    cost += _y as u32 * 100 + _x as u32;
                    bc += 1;
                }
            }
        }
        return cost;
    }

    fn coordinates_sum_2(&self) -> u32 {
        let mut bc: usize = self.box_cnt;
        let mut cost = 0u32;
        for _b in &self.boxes {
            cost += _b.pos[0].y as u32 * 100 + _b.pos[0].x as u32;
            bc += 1;
        }
        return cost;
    }
}

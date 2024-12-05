mod data;

struct CoordT {
    x: usize,
    y: usize,
    x_m: usize,
    y_m: usize,
}

struct DirT<'a> {
    x: isize,
    y: isize,
    #[allow(dead_code)]
    name: &'a str,
}

const UPWRD: DirT = DirT {
    x: 0,
    y: -1,
    name: "UPWRD",
};
const DOWRD: DirT = DirT {
    x: 0,
    y: 1,
    name: "DOWRD",
};
const LEFT_: DirT = DirT {
    x: -1,
    y: 0,
    name: "LEFT_",
};
const RIGHT: DirT = DirT {
    x: 1,
    y: 0,
    name: "RIGHT",
};
const DIAG1: DirT = DirT {
    x: 1,
    y: 1,
    name: "DIAG1",
};
const DIAG2: DirT = DirT {
    x: -1,
    y: 1,
    name: "DIAG2",
};
const DIAG3: DirT = DirT {
    x: 1,
    y: -1,
    name: "DIAG3",
};
const DIAG4: DirT = DirT {
    x: -1,
    y: -1,
    name: "DIAG4",
};

impl CoordT {
    pub fn new(x_m: usize, y_m: usize) -> CoordT {
        CoordT {
            x: 0,
            y: 0,
            x_m: x_m,
            y_m: y_m,
        }
    }
    #[allow(dead_code)]
    pub fn copy(&mut self) -> CoordT {
        CoordT {
            x: self.x,
            y: self.y,
            x_m: self.x_m,
            y_m: self.y_m,
        }
    }
    #[allow(dead_code)]
    pub fn forward(&mut self) -> bool {
        self.x += 1;
        if self.x == self.x_m {
            self.x = 0;
            self.y += 1;
            if self.y == self.y_m {
                self.y = 0;
                return false;
            }
        }
        return true;
    }
    #[allow(dead_code)]
    pub fn next(&mut self, dir: &DirT) -> bool {
        if (dir.x > 0) && (self.x == self.x_m - 1)
            || (dir.x < 0) && (self.x == 0)
            || (dir.y > 0) && (self.y == self.y_m - 1)
            || (dir.y < 0) && (self.y == 0)
        {
            return false;
        }
        self.x = self.x.checked_add_signed(dir.x).unwrap();
        self.y = self.y.checked_add_signed(dir.y).unwrap();
        return true;
    }
}

struct ArrT {
    cursor: CoordT,
    array: Vec<Vec<char>>,
}

impl ArrT {
    pub fn new(input: &str) -> ArrT {
        let mut array: Vec<Vec<char>> = Vec::new();
        for s in input.split("\n") {
            if s.len() != 0 {
                array.push(s.chars().collect());
                println!("{}", s);
            }
        }
        let cursor = CoordT::new(
            array[0].len().try_into().unwrap(),
            array.len().try_into().unwrap(),
        );
        return ArrT {
            cursor: cursor,
            array: array,
        };
    }
    pub fn get(&mut self, crs: &CoordT) -> char {
        assert!(crs.x < self.array[0].len());
        assert!(crs.y < self.array.len());
        return self.array[crs.y][crs.x];
    }
    pub fn find(&mut self, target: &String) -> u32 {
        let mut ret: u32 = 0;
        let all_directions: [DirT; 8] = [UPWRD, DOWRD, LEFT_, RIGHT, DIAG1, DIAG2, DIAG3, DIAG4];
        let mut _cursor = self.cursor.copy();
        let mut _buf = String::from("");

        for _dir in all_directions {
            _buf.clear();
            _cursor = self.cursor.copy();
            assert!(_buf.len() == 0);
            if self.get(&_cursor) != 'X' {
                continue;
            }
            _buf.push(self.get(&_cursor));
            while _cursor.next(&_dir) && _buf.len() < target.len() {
                _buf.push(self.get(&_cursor));
            }
            if _buf == *target {
                ret += 1;
            }
        }
        return ret;
    }
    pub fn xfind(&mut self, target: &String) -> u32 {
        let mut ret: u32 = 0;
        let all_directions: [DirT; 2] = [DIAG1, DIAG2];
        let mut _cursor_a = self.cursor.copy();
        let mut _cursor_b = self.cursor.copy();
        let mut _cursor_c = self.cursor.copy();
        let mut _buf = String::from("");

        for _dir in all_directions {
            let _rev = DirT {
                x: -_dir.x,
                y: -_dir.y,
                name: "rev",
            };
            _buf.clear();
            _cursor_a = self.cursor.copy();
            _cursor_b = self.cursor.copy();
            _cursor_c = self.cursor.copy();
            assert!(_buf.len() == 0);

            if _cursor_a.next(&_dir) && _cursor_c.next(&_rev) {
                _buf.push(self.get(&_cursor_a));
                _buf.push(self.get(&_cursor_b));
                _buf.push(self.get(&_cursor_c));
            }
            let reverse: String = target.chars().rev().collect();
            if _buf == *target || _buf == reverse {
                ret += 1;
            }
        }
        if ret == 2 {
            return 1;
        } else {
            return 0;
        }
    }
}

#[test]
fn test_004() {
    assert_eq!(xmascnt(&data::TEST), 18);
    assert_eq!(xdashmascnt(&data::TEST2), 9);
}

fn main() {
    println!("Hello, world!");
    println!("{}", xmascnt(&data::INPUT));
    println!("{}", xdashmascnt(&data::INPUT));
}

fn xmascnt(input: &str) -> u32 {
    let mut ret: u32 = 0;
    let mut array = ArrT::new(input);
    ret += array.find(&String::from("XMAS"));
    while array.cursor.forward() {
        ret += array.find(&String::from("XMAS"));
    }
    return ret;
}

fn xdashmascnt(input: &str) -> u32 {
    let mut ret: u32 = 0;
    let mut array = ArrT::new(input);
    ret += array.xfind(&String::from("MAS"));
    while array.cursor.forward() {
        ret += array.xfind(&String::from("MAS"));
    }
    return ret;
}

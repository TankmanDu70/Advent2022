use std::{borrow::BorrowMut, f32::consts::E, ops::Index};

mod data;

#[test]
fn test_aoc() {
    let mut fs = FileSysT::new(data::TEST);
    assert_eq!(fs.print(), data::FIRST9);
    fs.fragment();
    assert_eq!(fs.print(), data::SEC9);
    assert_eq!(fs.checksum(), 1928);

    let mut fs2 = FileSysT::new(data::TEST);
    fs2.fragment2();
    assert_eq!(fs2.print(), data::SEC10);
    assert_eq!(fs2.checksum(), 2858);
}

fn main() {
    println!("Hello, world!");
    let mut fs = FileSysT::new(data::DATA);
    fs.fragment2();
    println!("RESULT {}", fs.checksum());
}
#[derive(Clone, PartialEq, Eq)]
struct FileT {
    id: usize,
    start: usize,
    size: usize,
    is_void: bool,
}

impl FileT {
    pub fn new(id: usize, start: usize, size: usize, is_void: bool) -> FileT {
        FileT {
            id: id,
            start: start,
            size: size,
            is_void: is_void,
        }
    }
}

struct FileSysT {
    files: Vec<FileT>,
    checksum: u64,
    free_m: u32,
}

impl FileSysT {
    pub fn new(input: &str) -> FileSysT {
        let mut id = 0usize;
        let mut pos = 0usize;
        let mut free_m = 0u32;
        let mut files = Vec::new();
        for c in input.chars() {
            let is_mt = id % 2 == 1;
            let size = c.to_string().parse::<usize>().expect("parsing error");
            let mut name = id / 2;
            if is_mt {
                name = 0;
            }
            files.push(FileT::new(name, pos, size, is_mt));
            if id % 2 == 1 {
                free_m += 1;
            }
            pos += size;
            id += 1;
        }
        FileSysT {
            files: files,
            checksum: 0,
            free_m: free_m,
        }
    }
    pub fn checksum(&mut self) -> u64 {
        let mut res = 0u64;
        let mut carrier = 0u64;
        for f in &self.files {
            for _pos in 0..f.size {
                //println!(
                //    "{}+({}x{})={}",
                //    res,
                //    f.id as u64,
                //    carrier,
                //    res + (f.id as u64 * carrier)
                //);
                //println!("{}/{}", res, carrier);
                res = res + (f.id as u64 * carrier);
                carrier += 1;
            }
        }
        self.checksum = res;
        return res;
    }
    pub fn print(&self) -> String {
        let mut ret = String::from("");
        for f in &self.files {
            let mut name = f.id.to_string();
            if f.is_void {
                name = ".".to_string();
            }
            for _c in 0..f.size {
                ret.push_str(name.clone().as_str());
            }
        }
        //println!("{}", ret);
        return ret;
    }
    pub fn fragment(&mut self) {
        let mut nepos = 0usize;
        let mut end = self.files.len() - 1;
        for epos in 0..end {
            if !self.files[epos].is_void {
                continue;
            }
            nepos = self.last(false);
            if epos >= nepos {
                return;
            }
            let tnef = self.files.iter_mut().nth(nepos).unwrap().clone();
            let free_size = self.files[epos].size;
            if tnef.size == free_size {
                // file and space size are matching
                self.files[epos].id = tnef.id;
                self.files[epos].is_void = false;
                self.files[nepos].id = 0;
                self.files[nepos].is_void = true;
            } else if tnef.size > free_size {
                // the non-empty file at the end is bigger than the free splace chunk
                self.files[epos].is_void = false;
                self.files[epos].id = self.files.iter_mut().nth(nepos).unwrap().id;
                self.files.iter_mut().nth(nepos).unwrap().size -= free_size;
                let lf: usize = self.files.len() - 1;
                let last_is_void = self.files[lf].is_void;
                if last_is_void {
                    self.files[lf].size += free_size;
                } else {
                    self.files.push(FileT::new(0, 0, free_size, true));
                }
            } else if tnef.size < free_size {
                // the non-empty file fits entirely
                self.files[epos].id = tnef.id;
                self.files[epos].is_void = false;
                self.files[epos].size = self.files[nepos].size;
                self.files[nepos].is_void = true;
                self.files[nepos].id = 0;
                self.files.insert(
                    epos + 1,
                    FileT::new(
                        0,
                        self.files[epos].start + tnef.size,
                        free_size - tnef.size,
                        true,
                    ),
                );
            }
            //self.print();
        }
        end = self.files.len() - 1;
    }

    pub fn fragment2(&mut self) {
        let mut lnef = self.last(false);
        let fmtf = self.first(true);
        let mut len = self.files.len() - 1;

        while lnef != 0 && lnef > fmtf {
            //self.print();
            if self.files[lnef].is_void {
                lnef -= 1;
                continue;
            }
            for slot in fmtf..len {
                if !self.files[slot].is_void {
                    continue;
                }
                if slot > lnef {
                    break;
                }
                if self.files[slot].size > self.files[lnef].size {
                    self.files[slot].is_void = false;
                    self.files[slot].id = self.files[lnef].id;
                    let free_size = self.files[slot].size;
                    self.files[slot].size = self.files[lnef].size;
                    self.files[lnef].is_void = true;
                    self.files[lnef].id = 0;
                    self.files.insert(
                        slot + 1,
                        FileT::new(
                            0,
                            self.files[slot].start + self.files[lnef].size,
                            free_size - self.files[lnef].size,
                            true,
                        ),
                    );
                    len = self.files.len() - 1;
                    break;
                } else if self.files[slot].size == self.files[lnef].size {
                    self.files[slot].is_void = false;
                    self.files[slot].id = self.files[lnef].id;
                    self.files[lnef].id = 0;
                    self.files[lnef].is_void = true;
                    break;
                }
            }
            lnef -= 1;
        }
        //self.print();
    }

    pub fn last(&mut self, mt: bool) -> usize {
        let mut ret = self.files.len() - 1;
        for n in self.files.iter().rev() {
            if n.is_void != mt {
                if ret != 0 {
                    ret -= 1;
                }
                continue;
            }
            break;
        }
        return ret;
    }

    pub fn first(&mut self, mt: bool) -> usize {
        let mut ret = 0;
        for pos in 0..self.files.len() {
            if self.files[pos].is_void != mt {
                ret += 1;
            }
            break;
        }
        return ret;
    }
}

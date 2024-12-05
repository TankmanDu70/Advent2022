use std::collections::HashMap;
mod data;

#[test]
fn test_004() {
    let mut base = BaseT::new(data::TEST);
    assert_eq!(base.check_update(data::TUPDATES), 143);
    assert_eq!(base.fix_update(data::TUPDATES), 123);
}

fn main() {
    println!("Hello, world!");
    let mut base = BaseT::new(data::RULES);
    println!("{}", base.check_update(data::UPDATES));
    println!("{}", base.fix_update(data::UPDATES));
}

struct BaseT {
    entries: HashMap<u32, EntryT>,
}

impl BaseT {
    pub fn new(input: &str) -> BaseT {
        let mut entr = HashMap::new();
        for line in input.split("\n") {
            let left = line.split("|").collect::<Vec<_>>()[0]
                .parse::<u32>()
                .unwrap();
            let right = line.split("|").collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();
            //println!("{} must be before {}", left, right);

            if !entr.contains_key(&left) {
                let mut new_e: EntryT = EntryT::new(left);
                new_e.bfrs.push(right);
                entr.insert(left, new_e);
            } else {
                entr.get_mut(&left).unwrap().add_before(right);
            }
            if !entr.contains_key(&right) {
                let mut new_e: EntryT = EntryT::new(right);
                new_e.afts.push(left);
                entr.insert(right, new_e);
            } else {
                entr.get_mut(&right).unwrap().add_after(left);
            }
        }

        //for e in entr.values() {
        //    e.print();
        //}
        BaseT { entries: entr }
    }

    pub fn check_update(&mut self, input: &str) -> u32 {
        let mut result: u32 = 0;
        for line in input.split("\n") {
            let mut updates = Vec::new();
            for num in line.split(",") {
                updates.push(num.parse::<u32>().unwrap());
            }
            let mut ret: bool = true;
            for ii in 0..updates.len() {
                let val = updates.get(ii).unwrap();
                for jj in ii..updates.len() {
                    let afters = updates.get(jj).unwrap();
                    if val == afters {
                        continue;
                    }
                    ret = self.entries.get(afters).unwrap().brule(*val);
                    if !ret {
                        print!("{} is before {}", val, afters);
                        println!(", wich is {}", if ret { "good" } else { "bad" });
                        break;
                    }
                }
                if !ret {
                    break;
                }
                for kk in 0..ii {
                    let befores = updates.get(kk).unwrap();
                    if val == befores {
                        continue;
                    }
                    ret = self.entries.get(befores).unwrap().arule(*val);
                    if !ret {
                        print!("{} is after {}", val, befores);
                        println!(", wich is {}", if ret { "good" } else { "bad" });
                        break;
                    }
                }
                if !ret {
                    break;
                }
            }
            if !ret {
                continue;
            }
            println!(
                "{} is all good, adding {}",
                line,
                updates[updates.len() / 2]
            );
            result += updates[updates.len() / 2];
        }
        return result;
    }

    pub fn fix_update(&mut self, input: &str) -> u32 {
        let mut result: u32 = 0;

        for line in input.split("\n") {
            let mut updates = Vec::new();
            for num in line.split(",") {
                updates.push(num.parse::<u32>().unwrap());
            }
            let mut was_broken: bool = false;
            let mut ret: bool = false;
            //println!("Checking {:?}", updates);
            while !ret {
                for ii in 0..updates.len() {
                    let val = updates.get(ii).unwrap().clone();
                    for jj in ii..updates.len() {
                        let afters = updates.get(jj).unwrap().clone();
                        if val == afters {
                            continue;
                        }
                        ret = self.entries.get(&afters).unwrap().brule(val);
                        if !ret {
                            //print!("{} is before {}", val, afters);
                            //println!(", wich is {}", if ret { "good" } else { "bad" });
                            was_broken = true;
                            updates.swap(ii, jj);
                            break;
                        }
                    }
                    if !ret {
                        break;
                    }
                    for kk in 0..ii {
                        let befores = updates.get(kk).unwrap().clone();
                        if val == befores {
                            continue;
                        }
                        ret = self.entries.get(&befores).unwrap().arule(val);
                        if !ret {
                            //print!("{} is after {}", val, befores);
                            //println!(", wich is {}", if ret { "good" } else { "bad" });
                            was_broken = true;
                            updates.swap(ii, kk);
                            break;
                        }
                    }
                    if !ret {
                        break;
                    }
                }
                if !ret {
                    continue;
                }
            }
            println!(
                "{:?} is all good, adding {}",
                updates,
                updates[updates.len() / 2]
            );
            if was_broken {
                result += updates[updates.len() / 2];
            }
        }
        return result;
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct EntryT {
    val: u32,
    afts: Vec<u32>,
    bfrs: Vec<u32>,
}

impl EntryT {
    pub fn new(value: u32) -> EntryT {
        EntryT {
            val: value,
            afts: Vec::new(),
            bfrs: Vec::new(),
        }
    }
    pub fn arule(&self, other: u32) -> bool {
        for a in &self.afts {
            if other == *a {
                return false;
            }
        }
        return true;
    }
    pub fn brule(&self, other: u32) -> bool {
        for a in &self.bfrs {
            if other == *a {
                return false;
            }
        }
        return true;
    }
    pub fn add_before(&mut self, val: u32) {
        self.bfrs.push(val);
    }
    pub fn add_after(&mut self, val: u32) {
        self.afts.push(val);
    }
    pub fn print(&self) {
        println!("{} must be before {:?}", self.val, self.bfrs);
        println!("{} must be after {:?}", self.val, self.afts);
    }
}

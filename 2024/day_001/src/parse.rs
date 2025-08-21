use regex::Regex;
use std::collections::HashMap;

const CAP_LEFT_IND: usize = 1;
const CAP_RIGHT_IND: usize = 2;
const CAP_COUNT_IND: usize = 3;
const VEC_LEFT_IND: usize = 0;
const VEC_RIGHT_IND: usize = 1; 
 
pub fn common(input: &str) -> Vec<Vec<u64>> {
    let mut ret: Vec<Vec<u64>> = vec![];
    let dist_rgx = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
    let matches: Vec<&str> = dist_rgx.find_iter(input).map(|m| m.as_str()).collect();
    let mut lefts: Vec<u64> = vec![];
    let mut rights: Vec<u64> = vec![];

    for line in matches {
        let capture = dist_rgx.captures(line).unwrap();
        assert_eq!(capture.len(), CAP_COUNT_IND);
        lefts.push(
            capture
                .get(CAP_LEFT_IND)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
        );
        rights.push(
            capture
                .get(CAP_RIGHT_IND)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
        );
    }
    ret.push(lefts);
    ret.push(rights);

    return ret;
}
 
pub fn part_1(input: &str) -> u64 {
    let mut ret = 0u64;
    let mut vectors = common(input);

    vectors.get_mut(VEC_LEFT_IND).unwrap().sort();
    vectors.get_mut(VEC_RIGHT_IND).unwrap().sort();

    for _it in 0..vectors.get(0).unwrap().len() {
        let delta;
        let a = vectors.get(VEC_LEFT_IND).unwrap().get(_it).unwrap();
        let b = vectors.get(VEC_RIGHT_IND).unwrap().get(_it).unwrap();
        if a > b {
            delta = a - b;
        } else {
            delta = b - a;
        }
        ret += delta;
    }
    return ret;
}
 
pub fn part_2(input: &str) -> u64 {
    let mut ret = 0u64;
    let vectors: Vec<Vec<u64>> = common(input);
    let mut map = HashMap::<u64, u64>::new();

    for _it in 0..vectors.get(VEC_LEFT_IND).unwrap().len() {
        let right_val = vectors.get(VEC_RIGHT_IND).unwrap().get(_it).unwrap();
        if map.contains_key(right_val) {
            *map.get_mut(right_val).unwrap() += 1;
        } else {
            map.insert(right_val.clone(), 1);
        }
    }
    for _it in 0..vectors.get(VEC_LEFT_IND).unwrap().len() {
        let left_val = vectors.get(VEC_LEFT_IND).unwrap().get(_it).unwrap();
        if map.contains_key(left_val) {
            ret += left_val * map.get(left_val).unwrap();
        }
    }
    return ret;
}

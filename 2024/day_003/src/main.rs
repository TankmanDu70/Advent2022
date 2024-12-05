use regex::Regex;
mod data;

#[test]
fn test_003() {
    pub const TEST: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    pub const TEST2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(parse(&TEST), 161);
    assert_eq!(parse2(&TEST2), 48);
}

fn main() {
    println!("Hello, world!");
    parse(data::INPUT);
    parse2(data::INPUT);
}

fn parse(input: &str) -> u32 {
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let matches: Vec<&str> = reg.find_iter(input).map(|m| m.as_str()).collect();
    let mut ret = 0;

    for i in matches {
        let cap = reg.captures(i).unwrap();
        let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        ret += a * b;
    }
    println!("Ret1 {}", ret);
    return ret;
}

fn parse2(input: &str) -> u32 {
    let reg: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let matches: Vec<_> = reg.find_iter(input).map(|m| m.as_str()).collect();
    let mut ret = 0;
    let mut enabled: bool = true;

    for i in matches {
        let cap = reg.captures(i).unwrap();
        if cap.get(1).is_some() && cap.get(2).is_some() && enabled {
            let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            ret += a * b;
        } else if cap.get(0).is_some() {
            if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else {
                enabled = false;
            }
            continue;
        }
    }
    println!("Ret2 {}", ret);
    return ret;
}

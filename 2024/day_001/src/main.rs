mod data;
mod parse;

#[test]
fn test_aoc() {
    assert_eq!(parse::part_1(data::TEST), 11);
    assert_eq!(parse::part_1(data::DATA), 2057374);

    assert_eq!(parse::part_2(data::TEST), 31);
    assert_eq!(parse::part_2(data::DATA), 23177084);
}

fn main() {
    println!("Hello, world!");
    println!("part 1 = {}", parse::part_1(data::DATA));
    println!("part 1 = {}", parse::part_2(data::DATA));
}

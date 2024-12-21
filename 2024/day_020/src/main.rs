mod data;
mod deer_map;

#[test]
fn test_aoc() {
    let mut map = deer_map::Map::new(data::TEST);
    map.all_moves(false);
    map.printf(true);
    assert_eq!(map.passes, true);
    assert_eq!(map.best_score(), 84);
    let cheats = map.cheat(2);
    assert_eq!(cheats.get(&4).unwrap().len(), 14);
    assert_eq!(cheats.get(&10).unwrap().len(), 2);
    assert_eq!(cheats.get(&12).unwrap().len(), 3);
    assert_eq!(cheats.get(&20).unwrap().len(), 1);
    assert_eq!(cheats.get(&36).unwrap().len(), 1);
    assert_eq!(cheats.get(&38).unwrap().len(), 1);
    assert_eq!(cheats.get(&40).unwrap().len(), 1);
    assert_eq!(cheats.get(&64).unwrap().len(), 1);
    assert_eq!(cheats.get(&8).unwrap().len(), 4);
    assert_eq!(cheats.get(&6).unwrap().len(), 2);
    assert_eq!(cheats.get(&2).unwrap().len(), 14);

    let cheats = map.cheat_2(50);
    assert_eq!(cheats.get(&50).unwrap().len(), 32);
    assert_eq!(cheats.get(&52).unwrap().len(), 31);
    assert_eq!(cheats.get(&54).unwrap().len(), 29);
    assert_eq!(cheats.get(&56).unwrap().len(), 39);
    assert_eq!(cheats.get(&58).unwrap().len(), 25);
    assert_eq!(cheats.get(&60).unwrap().len(), 23);
    assert_eq!(cheats.get(&62).unwrap().len(), 20);
    assert_eq!(cheats.get(&64).unwrap().len(), 19);
    assert_eq!(cheats.get(&66).unwrap().len(), 12);
    assert_eq!(cheats.get(&68).unwrap().len(), 14);
    assert_eq!(cheats.get(&70).unwrap().len(), 12);
    assert_eq!(cheats.get(&72).unwrap().len(), 22);
    assert_eq!(cheats.get(&74).unwrap().len(), 4);
    assert_eq!(cheats.get(&76).unwrap().len(), 3);
}

fn main() {
    println!("Hello, world!");
    /*Adjust the deer direction (rection) before start*/
    let mut map = deer_map::Map::new(data::DATA);
    map.all_moves(false);
    let mut ans = 0;
    for _c in map.cheat(2) {
        if _c.0 >= 100 {
            ans += _c.1.len();
        }
    }
    println!("Part 1 :{}", ans);

    for _c in map.cheat_2(100) { 
        if _c.0 >= 100 {
            ans += _c.1.len(); 
        }
    }
    println!("Part 2 :{}", ans);
}

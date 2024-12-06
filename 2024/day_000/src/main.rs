use std::collections::HashMap;
mod data;

#[test]
fn test_004() { 
    assert_eq!(data::TEST, "test");
    assert_eq!(data::DATA, "data");
}

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
pub const TEST: &str = "test";
/**
 * 2,4  A % 8 -> B
 * 1,3  B ^ 3 -> B
 * 7,5  A / 2 pow B -> A
 * 0,3  A / 2 pow 3 -> A 
 * 1,5  B ^ 5 -> B
 * 4,4  B ^ C -> B
 * 5,5  B % 8 -> output
 * 3,0  A == 0 ? nop : 0 -> i_p
 */
 #[allow(dead_code)]
pub const DATA: &str = "Register A: 55593699
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0"; // 16 chars, so A must be big enough so as to jump back 15 times 
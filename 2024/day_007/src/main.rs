mod data;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
mod parser;

use std::time::SystemTime;

#[test]
fn test_007() {
    assert_eq!(parse_input(data::TEST, 6), 11387);
}

fn parse_input(str: &str, len: usize) -> u64 {
    let mut calculon = parser::CalculatorT::new(len);
    calculon.combinator(len);
    let mut res: u64 = 0;
    for s in str.split("\n") {
        println!("Trying {s}");
        let eq = parser::Equation::parse(s).unwrap();
        if eq.test(&mut calculon).unwrap() {
            res += eq.get_result();
        }
    }
    println!("END RESULT = {}", res);
    return res;
}

fn get_highest_cnt(str: &str) -> usize {
    let mut res: usize = 0;
    for s in str.split("\n") {
        let eq = parser::Equation::operands_cnt(s) as usize;
        if eq > res {
            res = eq;
        }
    }
    println!("END RESULT = {}", res);
    return res;
}

fn main() {

    println!("Hello, world!");
    let sys_time = SystemTime::now();
    let new_sys_time = SystemTime::now();

    let len = get_highest_cnt(data::DATA);
    parse_input(data::DATA, len);

    let difference = new_sys_time
        .duration_since(sys_time)
        .expect("Clock may have gone backwards");
    println!("{difference:?}");



    // Get an output stream handle to the default physical sound device.
    // Note that no sound will be played if _stream is dropped
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("/home/thomas/Music/windows-xp-shutdown.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).expect("FAILED");

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}

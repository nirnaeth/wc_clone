use std::fs::File;
use std::fs::read_to_string;

fn main() {
    let options = std::env::args().nth(1);
    let filename = std::env::args().nth(2).expect("no file given");

    if options == Some("-c".to_string()) {
        let file = File::open(filename.clone()).expect("could not open file");
        let byte_length = file.metadata().unwrap().len();
        println!("{} {}", byte_length, filename);
        return;
    } else if options == Some("-l".to_string()) {
        let line_count = read_to_string(filename.clone()).unwrap().lines().count();
        println!("{} {}", line_count, filename);
        return;
    }
}

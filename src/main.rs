use std::fs::File;

fn main() {
    let options = std::env::args().nth(1);
    let filename = std::env::args().nth(2).expect("no file given");
    let file = File::open(filename.clone()).expect("could not open file");

    if options == Some("-c".to_string()) {
        let byte_length = file.metadata().unwrap().len();
        println!("{} {}", byte_length, filename);
        return;
    }
}

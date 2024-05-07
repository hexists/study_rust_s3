use std::fs::{self, File};
use std::io::Write;

fn main() {
    let filename = "FizzBuzz.txt";
    {
        let mut fp = File::create(filename).unwrap();
        let mut data = String::new();

        // for i in 1..101 {
        for i in 1..=100 {
            let mut line = format!("{}\n", i);
            if i % 3 == 0 && i % 5 == 0 {
                line = String::from("FizzBuzz\n");
            }
            else if i % 3 == 0 {
                line = String::from("Fizz\n");
            }
            else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }
            data += &line;
        }

        let bytes = data.as_bytes();
        fp.write_all(bytes).unwrap();
    }
    let read_file = fs::read_to_string(filename).unwrap();
    println!("{}", read_file);
}

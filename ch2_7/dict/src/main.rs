use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dictfile = "dict.txt";

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] dict word");
        return;
    }

    let word = &args[1];
    println!("{:?}", word);

    let fp = File::open(dictfile).unwrap();
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.find(word) == None {
            continue;
        }
        println!("{}", line);
    }
}

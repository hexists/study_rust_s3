use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요");
    }

    let filename = &args[1];
    // let text = fs::read_to_string(filename).unwrap();
    let text = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let mut sum = 0;
    // println!("{}", text);
    for line in text.split("\n") {
        // println!("{}", line);

        let num:i32 = match line.parse() {
            Ok(line) => line,
            Err(_) => 0,
        };
        sum += num;
    }
    println!("{:?}", sum);
}

fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    let mut sum = 0;

    for (i, arg) in args.enumerate() {
        if i == 0 {
            continue;
        }

        let num:i32 = match arg.parse() {
            Ok(arg) => arg,
            Err(_) => 0,
        };
        sum += num;
    }
    println!("{:?}", sum);
}

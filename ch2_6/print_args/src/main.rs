fn main() {
    /*
    let args = std::env::args();
    for (i, arg) in args.enumerate() {
        if i == 0 {
            continue;
        }

        println!("{}", arg);
    }*/

    let args:Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}

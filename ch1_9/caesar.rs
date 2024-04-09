fn caesar(a:char, shift:i16) -> char {
    let na = a as i16;
    if a.is_alphabetic() {
        ((na + shift) as u8) as char
    }
    else {
        a
    }
}

fn main() {
    let text = "I LOVE RUST.";
 
    for ch in text.chars() {
        let nch = caesar(ch, 3);
        // println!("{} => {}", ch, nch);
        print!("{}", nch);
    }
    println!();
    
}

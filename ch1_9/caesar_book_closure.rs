fn encrypt(text: &str, shift: i16) -> String {
	let code_a = 'A' as i16;
	let is_az = |c| 'A' <= c && c <= 'Z';
	let conv = |c| (((c-code_a+shift+26)%26+code_a) as u8) as char;
	let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    let mut result = String::new();
    for ch in text.chars() {
        result.push(enc1(ch));
    }
    return result;
}

fn main() {
    let inp = "I LOVE RUST.";
    let enc = encrypt(&inp, 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}

fn main() {
	for i in 1..=9 {
		for j in 1..=9 {
			print!("{:3}", i * j);
			if j < 9 {
				print!(", ");
			}
		}
		println!();
	}
}

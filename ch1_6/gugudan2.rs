fn main() {
	for i in 1..=9 {
		let dan = (1..=9)
			.map(|j| format!("{:3}", i * j))
			.collect::<Vec<String>>().join(", ");
		println!("{}", dan);
	}
}

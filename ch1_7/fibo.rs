fn main() {
	let mut a = 1;
	let mut b = 2;
	println!("{}", a);
	println!("{}", b);

	for _ in 1..11 {
		println!("{}", a + b);
		// a, b = b, a + b
		let tmp = a;
		a = b;
		b = tmp + b;
	}
}

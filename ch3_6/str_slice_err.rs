fn main() {
	let pr = "🐶🐱🐭🐻";
	// 앞의 2글자(6바이트)를 얻기
	println!("앞 2글자: {}", &pr[0..6]);
}

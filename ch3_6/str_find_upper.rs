fn main() {
	// 변수 s에 문장을 대입
	let s = format!("{}{}", 
					"There is more happiness in giving ", 
					"than there is in receiving.");
	// 클로저를 검색
	let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
	match res {
		Some(i) => println!("S={}B", i),
		None => println!("None"),
	};
}

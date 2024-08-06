fn main() {
	let s = "내 자신에 대한 자신감을 잃으면 온 세상이 나의 적이 된다.";
	// 문자열 치환
	let s2 = s.replace("잃으면", "가지면");
	let s3 = s2.replace("적이", "편이");
	// 치환 전과 후를 출력
	println!("수정 전 : {}\n수정 후 : {}", s, s3);

	print_type_of(&s);
	print_type_of(&s2);
	print_type_of(&s3);
}

fn print_type_of<T>(_: &T) {
	// https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
	println!("{}", std::any::type_name::<T>())
}

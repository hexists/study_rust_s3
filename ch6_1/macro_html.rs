// 재귀적으로 HTML 구조를 출력하는 매크로
macro_rules! out_html {
	// 인수가 없을 때
	() => {()};
	// 인수가 1개일 때
	($e:tt) => {print!("{}", $e)};
	// 태그 [ 안쪽 ]을 계속 지정하는 경우
	($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
		print!("<{}>", stringify!($tag));
		out_html!($($inner)*);
		println!("</{}>", stringify!($tag));
		out_html!($($rest)*);
	}};
}

fn main() {
	out_html!(
		html [
			head[title["test"]]
			body[
				h1["test"]
				p ["This is test."]
			]
		]
	)
}

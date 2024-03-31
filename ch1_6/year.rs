/*
태조 원년(1392년)부터 세종 32년(1450년)까지의 연호를 서력과 함께 나타낸다.
태조는 1392년부터, 정종은 1399년부터, 태종은 1401년부터, 세종은 1419년부터 시작한다.
연호의 1년째는 '원년'으로 표시한다.
*/

fn main() {
	for year in 1392..=1450 {
		print!("서력 {}년 = ", year);
		if 1392 <= year && year <= 1398 {
			if year == 1392 {
				println!("태조 원년");
			}
			else {
				println!("태조 {}년", year - 1392 + 1);
			}
		}
		else if 1399 <= year && year <= 1400 {
			if year == 1399 {
				println!("정조 원년");
			}
			else {
				println!("정조 {}년", year - 1399 + 1);
			}
		}
		else if 1401 <= year && year <= 1418 {
			if year == 1401 {
				println!("태종 원년");
			}
			else {
				println!("태종 {}년", year - 1401 + 1);
			}
		}
		else {
			if year == 1419 {
				println!("세종 원년");
			}
			else {
				println!("세종 {}년", year - 1419 + 1);
			}
		}
	}
}

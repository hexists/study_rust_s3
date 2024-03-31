/*
어떤 컴퓨터 업체의 PC 가격은 98만원이다. A 쇼핑몰에서는 배송비가 12000원이고 20% 할인된 가격으로 판매하고 있다. B 쇼핑몰에서는 배송료가 무료지만 10% 할인된 가격으로 팔고 있다. 두 쇼핑몰 중 어느 쇼핑몰이 싼지 계산하시오.
*/

fn main() {
	let price = 980000.0;
	let a_send_price = 12000.0;
	let a_discount = 0.2;
	let b_send_price = 0.0;
	let b_discount = 0.1;

	let a_final_price = price - (price * a_discount) + a_send_price;
	let b_final_price = price - (price * b_discount) + b_send_price;

	if a_final_price < b_final_price {
		println!("a쇼핑몰이 더 쌉니다.")
	}
	else {
		println!("b쇼핑몰이 더 쌉니다.")
	}
	println!("a쇼핑몰 최종 가격: {}원\nb쇼핑몰 최종 가격: {}원", a_final_price, b_final_price);
}

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


mod cipher_str;


#[wasm_bindgen]  // 모듈 내 함수를 자바스크립트에서 사용할 수 있도록 지정
pub fn encrypt(password: &str, data: &str) -> String {
	cipher_str::encrypt(password, data)
}

#[wasm_bindgen]
pub fn decrypt(password: &str, data: &str) -> String {
	cipher_str::decrypt(password, data)
}

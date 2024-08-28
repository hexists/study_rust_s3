use std::time::{SystemTime, UNIX_EPOCH};

// 난수 초기값을 만드는 함수
fn rand_init() -> u32 {
  /* seed 초기화 작업 : 이곳을 채워주세요! */
  let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
  return epoc.as_millis() as u32;
}

// start ~ end 사이 난수 생성
fn rand(seed: &mut u32, start: u32, end:u32) -> u32 {
  *seed ^= *seed << 13;
  *seed ^= *seed << 17;
  *seed ^= *seed << 5;
  return *seed % (end - start + 1) + start;
}

fn main() {
  /* seed 변수 선언 : 이곳을 채워주세요! */
  let mut seed = rand_init();
  for _ in 0..100 {
    // 난수 생성 
    /* 함수 호출 : 이곳을 채워주세요! */
	let v = rand(&mut seed, 1, 6);
    println!("{}", v);
  }
}

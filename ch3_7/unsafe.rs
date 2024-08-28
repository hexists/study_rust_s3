use std::time::{SystemTime, UNIX_EPOCH};

// seed 를 전역 변수로 선언
/* 전역 변수 선언 : 이곳을 채워주세요! */
static mut SEED: u32 = 10;

// start ~ end 사이 난수 생성
/* 함수 정의 : 이곳을 채워주세요! */ 
unsafe fn rand(start: u32, end: u32) -> u32
{
  if SEED == 0 {
    // 시간으로 seed 초기화 (time.time() - 초 단위로 반환)
    let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    SEED = epoc.as_millis() as u32;
  }
  SEED ^= SEED << 13;
  SEED ^= SEED << 17;
  SEED ^= SEED << 5;
  return SEED % (end - start + 1) + start;
}

fn main() {
  for _ in 0..100 {
    unsafe {
    // 난수 생성 
    /* 함수 호출 및 출력 : 이곳을 채워주세요! */ 
	let v = rand(1, 6);
    print!("{}", v);
    }
  }
}

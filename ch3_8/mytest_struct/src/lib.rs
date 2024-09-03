// 구조체 정의
#[derive(Debug, PartialEq)] // Debug는 formatter를 위한 것, PartialEq는 구조체의 각 요소를 비교할 수 있게 하는 지시자.
struct GItem {
  name: String,
  price: i64,
}

#[cfg(test)]
mod tests{
  use super::*; // 모듈 밖의 요소를 이용한다는 것을 선언
  #[test]
  fn item_test(){
    // 구조체 초기화
    let apple1 = GItem{
      name: String::from("사과"),
      price: 2400,
    };
    
    let mut apple2 = GItem{
      name: "사과".to_string(),
      price: 0,
    };
    
    apple2.price = 2400;
    
    // 구조체 필드 비교
    assert_eq!(apple1.name, apple2.name);
    assert_eq!(apple1.price, apple2.price);
    
    // 구조체 자체를 비교 PartialEq 지시자가 없다면 아래 코드에서 에러가 발생
    // binary operation `==` cannot be applied to type `GItem`
    assert_eq!(apple1, apple2);
  }
}

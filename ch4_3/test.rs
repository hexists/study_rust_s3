fn main() {
    // i32 타입 Vec을 사용
    let mut v1: Vec<i32> = Vec::<i32>::new();
    v1.push(10); // 요소를 v1에 추가 --- (*1a)
    v1.push(20);
    v1.push(30);
    v1.pop(); // 마지막 요소를 꺼내기 --- (*1b)
    // v1 변수의 요소를 하나씩 출력 --- (*1c)
    for i in v1.iter() {
        println!("{}", i);
    }

    // char 타입 Vec을 사용
    // Vec v2 생성
    let mut v2: Vec<char> = Vec::<char>::new();
    
    // Vec에 요소를 추가 -'a', 'b', 'c'
	v2.push('a');
	v2.push('b');
	v2.push('c');
    
    // 마지막 요소를 꺼내기
	v2.pop();

    // v2 변수의 요소를 하나씩 출력
    for i in v2.iter() {
        println!("{}", i);
    }
}

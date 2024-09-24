// 보물 상자의 동작을 정의하는 트레잇
// 1. open, 2. check
trait TreasureBox {
    fn open(&self, key_no:i32) -> bool {
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

// 쥬얼리 박스
struct JewelryBox {
    price : i32, // 몇 골드가 들어 있는가
    key_no: i32, // 몇번째 열쇠가 있어야  열리는가?
}

// 구현
impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("보석 상자였다. {} 골드 입수", self.price);
    }

    fn get_key_no(&self) -> i32 {
        return self.key_no
    }
}

struct EmptyBox {
    key_no: i32,
}

// 구현
impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("{}번 박스가 비어 있습니다.", self.key_no);
    }

    fn get_key_no(&self) -> i32 {
        return self.key_no
    }
}

// 모험가가 상자를 여는 동작
fn check_box(tbox: &impl TreasureBox, key_no:i32) {
    if !tbox.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
        return;
    }
    tbox.check();
}

// JewelryBox와 EmptyBox의 데이터 타입이 같아야함(struct)

fn main() {
    // 다양한 상자를 준비

    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };

    let box2 = EmptyBox {key_no: 2};
    let box3 = JewelryBox {
        price: 20,
        key_no: 2,
    };

    // 모험가가 가진 열쇠로 상자를 연다.
    let my_key = 2;

    check_box(&box1, my_key);
    check_box(&box2, my_key);
    check_box(&box3, my_key);

}


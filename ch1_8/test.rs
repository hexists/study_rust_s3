fn main() {
    let price = 3950;
    for a in 0..=10 {  // 500
        for b in 0..=3 {  // 100
            for c in 0..=10 {  // 50
                let total = a * 500 + b * 100 + c * 50;

                if price == total {
                    println!("500원x{} + 100원x{} + 50원x{}", a, b, c);
                }
            }
        }
    }
}

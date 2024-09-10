struct Body {
    height: f64,
    weight: f64
}

impl Body {
    fn calc_bmi(&self) -> f64 {
        let h  = self.height / 100.0;
        return self.weight / h.powf(2.0)
    }

    fn calc_per(&self) -> f64 {
        return self.calc_bmi() / 22.0 * 100.0
    }
}

fn main() {
    // 구조체에 값을 넣어 객체화
    let yang = Body {
        height: 160.0,
        weight: 70.0
    };

    println!("BMI = {:.2}", yang.calc_bmi());
    println!("비만률 = {:.1}", yang.calc_per());
}

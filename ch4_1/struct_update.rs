#[derive(Clone)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Person {
            name: name.to_string(),
            age
        }
    }
}

fn main() {
    let alex = Person::new("Alex", 18);
    let name = String::from("Betty");
    let betty = Person {
        name,
        ..alex
    };

    println!("{}, {}", alex.name, alex.age);
    println!("{}, {}", betty.name, betty.age);

    let mut casey = alex.clone();
    casey.name = "Casey".to_string();
    println!("{}, {}", casey.name, casey.age);
}

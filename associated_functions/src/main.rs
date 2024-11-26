struct Person {
    id: i32,
    name: String,
    active: bool,
}
impl Person {
    // 연관함수
    // Person 구조체의 인스턴스를 생성하며 리턴
    fn new(id: i32, name: String) -> Person {
        Person {
            id,
            name,
            active: true,
        }
        // 메서드
    }
}

fn main() {
    // 연관함수 호출 Person::new()
    let p = Person::new(101, String::from("Tom"));
    println!("{} : {}", p.id, p.name)
}

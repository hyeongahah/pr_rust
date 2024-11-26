// 구조체 선언 - struct 는 타입스크립트의 interface, type와 유사하다.
// 일련의 연관된 데이터들의 그룹을 만들 목적 / Custom data type

fn main() {
    mem_1();
    color_1();
    unit_like();
    // 문자열 리터럴은 기본적으로 `&str` 타입이므로, 소유권을 가진 `String` 타입으로 변환하기 위해 `to_owned()` 메서드를 사용
    let mem = get_member("Kim".to_owned(), "Park".to_owned(), 33);
    println!("{}", mem.fname)
}

//일반 구조체 형태
struct Member {
    active: bool,
    fname: String,
    lname: String,
    age: u16,
}
fn mem_1() {
    let mut mem1 = Member {
        active: true,
        fname: String::from("Tom"),
        lname: String::from("Lee"),
        age: 35,
    };
    mem1.active = false;
    println!(
        "{} {}, {} {}",
        mem1.fname, mem1.lname, mem1.age, mem1.active
    );
}

/* Tuple Struct : 튜플처럼 필드명을 갖지 않고, 필드들을 튜플로 묶어 정의하는 구조체
일반적인 구조체 = struct 구조체명 { 필드명 : 필드 타입 }, 튜플 구조체 = struct 구조체명 (필드 타입1, 필드타입 2) */
struct Color(u8, u8, u8);

fn color_1() {
    let red = Color(255, 0, 0);
    println!("R={}, G={}, B={}", red.0, red.1, red.2);
}

// Unit 타입은 ()으로 아무런 데이타를 갖지 않는 상태
// 구조체의 필드들을 정의하지 않는 것을 Unit-like Struct 라고 한다.
struct Dummy;
fn unit_like() {
    let _dummy = Dummy;
    //...
}

// 세 개의 매개변수를 받아서 Member 구조체로 반환
fn get_member(fname: String, lname: String, age: u16) -> Member {
    Member {
        fname: fname,
        lname, // 필드명과 파라미터명이 같으면 한번만 지정해도 된다. 자바스크립트에도 있는 기능
        age,
        active: true,
    }
}

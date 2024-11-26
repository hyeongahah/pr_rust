fn main() {
    arrays();
    tuples();
    str_type();
    string_type();
}
// 배열은 동일한 데이타 타입만 갖는다. 러스트에서 배열은 스택에 할당된다.
fn arrays() {
    let arr: [i32; 3] = [1, 2, 3];
    println!("{}", arr[0]); // 배열 첫번째 요소 출력
    println!("{:?}", arr); // 배열 전체를 출력
}
// 튜플은 여러 데이타 타입의 값을 하나로 묶은 것으로, 한번 정의되면, 새로운 요소를 추가하거나 기존 요소를 삭제할 수 없다.
fn tuples() {
    let dat: (i32, char, bool) = (1, 'A', true);
    let _usr = ("Tom", 'B'); // 문자열과 문자 타입
    let a = dat.0;
    let b: char = dat.1;
    let _c: bool = dat.2;
    let (_d, _e, f) = dat; // Destructuring
    println!("{}", a);
    println!("{}", b);
    println!("{}", f);
}
/* &str 타입은 문자열을 표현하는 Primitive 문자열 타입 */

fn str_type() {
    let s = "hello"; // let s: &'static str = "hello"; 와 동일

    // 변수 s는 &'static str 타입으로 문자열 리터럴 "hello" 에 대한 레퍼런스(reference)이다. &str의 &는 레퍼런스를 의미하고, 여기서 문자열 리터럴 전체를 가리킨다.

    let sub: &str = &s[1..4]; // 요소 1부터 3까지 - 시작 이상, 끝 미만 / 문자열 슬라이스

    /*  &str 타입을 String 타입으로 변환하기 위해서는 to_owned() 함수를 사용한다. (주: to_string()은 임의의 타입을 String 타입으로 변환하는 것인데, 더 많은 메모리를 사용하므로 &str 타입에서는 to_owned() 함수를 사용하는 것이 좋다) */
    let x: String = sub.to_owned();
    println!("{}", s);
    println!("{}", sub);
    println!("{}", x);
    println!("{}", s.trim().to_uppercase()); //str 타입의 메서드 적용
}

/* String(std::string::String) 타입은 Heap 메모리 상에 문자열을 저장하는 타입으로, 문자열 메모리에 추가하거나 수정할 수 있다.(growable String) */
fn string_type() {
    let s = String::from("hello"); // 문자열 리터럴을 String 타입으로
                                   // let s : String = "hello".to_own
    println!("{}", s);

    /* 문자 및 문자열 추가하기 */
    let mut plus = String::new(); // empty string 생성, 가변성을 위해 mut
    plus.push('H');
    plus.push('I'); // 문자 추가
    plus.push_str(" Tom"); // 문자열 추가
    println!("{}", plus);
    /* 문자열 변환하기 */
    plus = plus.replace("Tom", "Bob");
    println!("{}", plus);
    /* 공백을 기준으로 분리하기 */
    // contains 괄호 안 문자, 공백 등이 있는지 확인하는 함수
    if plus.contains(" ") {
        for w in plus.split_whitespace() {
            println!("{}", w);
        }
    }
}

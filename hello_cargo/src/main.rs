fn main() {
    println!("Hello, world!");
    // 느낌표가 붙어있으면 매크로, 매크로는 변수, 다수의 매개변수, 다른 유형으로 처리 가능
    // Number : 13232, String : apple 문자열과 숫자를 동시에 출력함
    println!("Number : {}, String : {}", 13232, "apple");
}

// cargo-expand를 실행하면 아래와 같이 매크로가 확장한 모드 코드를 볼 수 있음
/*
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
    {
        ::std::io::_print(format_args!("Number : {0}, String : {1}\n", 13232, "apple"));
    };
}
     */

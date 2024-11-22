fn main() {
    let mut x = 5; // mut를 붙이면 변수 재할당 가능 / 러스트는 기본적으로 불변 변수
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
    reallocation();
}

/*
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 절대 불변 상수의 이름은 대문자 + _ 로 쓰는 것이 관례
*/

/* 섀도잉 - 새 변수를 이전 변수명과 같은 이름으로 선언할 수 있다 */
fn reallocation() {
    let y = 5;
    let y = y + 1; // let 키워드 없이 변수에 재할당 할 수 없다.  // 6
    let spaces = "같은 변수 이름  ";
    println!("이름 : {spaces}");
    let spaces = spaces.len();
    println!("크기 : {spaces}"); // 변수명은 같지만 위는 문자 타입이고, 아래는 숫자 타입이라서 구분되는 변수명을 안 쓸 수 있다
                                 // 하지만 mut를 붙일 경우, 타입이 다른 변수를 받는 걸 허용하지 않기 때문에 오류가 난다

    {
        let y = y * 2;
        println!("The value of y in the inner scope is : {y}") // 12
    }
    println!("The value of x is: {y}"); // 6
}

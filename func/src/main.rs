fn main() {
    add(2, 3);
    expression_ex();
    if_func();
    loop_func();
    while_func();
    for_func();
    continue_func();
}

fn add(a: i32, b: i32) -> i32 {
    dbg!(a, b);
    return a + b;
}

fn expression_ex() {
    let number = 13;
    let odd = if number & 2 == 0 { "even" } else { "odd" };
    println!("{}: {}", number, odd);
}

fn if_func() {
    // 비교는 ASCII 값으로 비교하기 때문에 A 는 B 보다 작다
    let score = 'B';
    if score <= 'B' {
        println!("Excellent")
    } else if score <= 'C' {
        println!("Pass");
    } else {
        println!("Fail");
    }
    // if나 else 블럭에 Statement를 쓸 수 있을 뿐만 아니라, Expression도 추가 가능
    // 다만 동일한 변수에 Expression 값을 리턴하여 변수에 넣기 위해서는 타입이 동일해야 한다.
    let ok = if score <= 'C' {
        println!("{}", score); // statement
        "Pass" // expression
    } else {
        "Fail" // expression
    };
    println!("{}", ok)
}

// 단순 반복
fn loop_func() {
    let mut i = 1;
    loop {
        if i > 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }
}

// 조건이 참일 때에만 반복
fn while_func() {
    let mut sum = 0;
    let mut i = 1;
    while i <= 10 {
        sum += i;
        i = i + i;
    }
    println!("sum of 1~10 : {}", sum);
}

// 배열이나 벡터와 같은 컬렉션으로부터 각 요소를 하나씩 가져와 루프를 돌며 처리할 때 사용
fn for_func() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("{}", i);
    }
    let mut sum = 0;
    // 숫자를 .. 앞뒤로 쓰면 시작부터 미만이 된다.
    for i in 1..101 {
        sum += i;
    }
    println!("sum of 1 ~ 100: {}", sum);
}

fn continue_func() {
    let mut i = 0;
    let mut sum = 0;
    let result = loop {
        i += 1;
        if i % 2 == 1 {
            continue;
        }
        sum += i;
        if i == 10 {
            break sum / 2;
        }
    };
    println!("{}", result);
}

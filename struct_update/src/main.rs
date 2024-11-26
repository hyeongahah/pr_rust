struct Admin {
    name: String,
    group: String,
}

fn main() {
    let adm1 = Admin {
        name: String::from("Tom"),
        group: String::from("IT"),
    };

    let adm2 = Admin {
        name: String::from("Kim"),
        ..adm1 // struct update syntax - 자바스크립트에도 유사 기능이 있음
               // adm1 구조체의 값을 가져올 때, 소유권을 가진 데이터 타입(Owned type)에 대해서는 소유권이 이동
    };

    println!("{}", adm2.group);
    println!("{}", adm1.name);
    println!("{}", adm2.name);
    // println!("{}", adm1.group);  소유권이 이동하여 사용할 수 없다. 에러남
    // adm1.name의 경우는 소유권이 이전되지 않았음
}

use std::io; // 표준 라이브러리 크레이트의 IO 모듈을 사용

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    // mars_weight는 자체적으로 할당 받은 caculate 함수를 보고 유형을 정한다. 물론 직접 지정해줘도 된다.
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}
// 함수 시그니처는 항상 각 매개변수의 유형을 명시해야 한다. (weight: f32) 'float32bit'
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

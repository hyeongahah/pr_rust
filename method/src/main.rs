struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        //여기에서 self는 Rectangle 구조체를 뜻함
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        // 두 도형의 크기를 비교해서 앞의 도형이 더 크면 true, 아니면 false
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    //rect1.width 뒤에 괄호를 붙이면 메서드로 인식, 괄호를 붙이지 않으면 width 필드를 의미
    println!("The rectangle has s nonzero width; it is {}", rect1.width());
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rec1 hold rect2? {}", rect1.can_hold(&rect2));
}

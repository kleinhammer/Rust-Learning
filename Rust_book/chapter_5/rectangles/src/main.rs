struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height:50,
    };

    let rect2 = Rectangle {
        width: 34,
        height:55,
    };

    // let rect3 = Rectangle {
    //     width: 29,
    //     height:51,
    // };
    println!("The area of the rectangle is { } square pixels.", rect1.area());

    println!("Can rect1 hold rect2?: { }", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1?: { }", rect2.can_hold(&rect1));
}
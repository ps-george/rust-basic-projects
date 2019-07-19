#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width:size, height:size}
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 20, height: 40 };
    let rect3 = Rectangle { width: 20, height: 60 };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("{:?}", rect1);
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(5);
}

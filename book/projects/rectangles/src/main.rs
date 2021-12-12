#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {  // Methods and fields can share same name
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }                                                                                             
}


fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;
    
    let rect1: (u32, u32) = (30, 50);

    let rect2 = Rectangle {
        width: dbg!(30),
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };
    let square1 = Rectangle::square(10);

    

    println!("The area of the rectangle is {} pixels^2", area(width1, height1));
    println!("The area of the rectangle is {} pixels^2", area_tuple(rect1));
    println!("The area of the rectangle is {} pixels^2", area_struct(&rect2));
    dbg!(&rect2);
    println!("The area of the rectangle is {} pixels^2", rect2.area());
    println!("The rectangle has a width: {}", rect2.width());
    println!("The rectangle rect2 can hold rect3: {}", rect2.can_hold(&rect3));
    println!("The rectangle rect2 can hold rect4: {}", rect2.can_hold(&rect4));
    println!("The square1: {:#?}", square1);
    
}

fn area(w: u32, h:u32) -> u32 {
    w * h
}

fn area_tuple(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}


fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

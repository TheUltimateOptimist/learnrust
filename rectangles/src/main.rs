// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels.",
//             area(width1, height1)
//         );

// }

// fn area(width: u32, height: u32) -> u32{
//     width*height
// }


//refactored with tuples
// fn main(){
//     let rect1 = (30, 50);

//     println!("The area of the rectangle is {} square pixels", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0*dimensions.1
// }


//refactored with structs
#[derive(Debug)]//enables pretty printing
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//multiple impl block possible
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);//all in one line
    println!("rect1 is {:#?}", rect1);//across multiple lines, useful for larger structs
    if rect1.width() {
        println!("The ractangle has a nonzero width; it is {}", rect1.width);
    }
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rectOne = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rectOne.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rectOne.can_hold(&rect3));
}

fn debugPrinting(){
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale), //dbg gives ownership back
        height: 50,
    };
    dbg!(&rect1);//reference so that dbg does not take ownership
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Point(u32, u32, String);

impl Rectangle{
    fn area(&self) -> u32{
        self.width *  self.height
    }

    fn can_hold(&self,  rectangle:&Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rectangle =  Rectangle{
        width:30,
        height:30
    };


    println!("in bound area {} of  {:#?}", rectangle.area() , rectangle);

    let rect1 =  Rectangle{
        width:  30,
        height:  40
    };
    let rect2 =  Rectangle{
        width:  60,
        height: 70
    };

    println!("is rect_two {:#?} greater rect_one {:#?}  {}", rect2, rect1, rect2.can_hold(&rect1));

    println!("square {:?}", Rectangle::square(34));

    let coords =  Point(55, 32,  String::from("this val"));

    println!("Here is the value of coords {:?}", coords  );
}

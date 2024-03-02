#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    let width1  =  30;
    let height1  =  50;


    println!("The area of the rectangle is {} squared pixels", area(width1, height1));


    const RECT: (u32, u32) = (30,50);

    println!("using tuples {}",  tuple_area(RECT));

    let rect = Rectangle{
        width:  30,
        height:30
    };


    println!("so the area using struct equals: {}",  calc_area(&rect));

    let rect1 =  Rectangle{
        width:  dbg!(30 + 1),
        height:  30
    };


    println!("the debug value of the rectangle = {:#?}",  rect1);
}


fn area(width: u32,  height:u32) -> u32{
    width * height
}

fn tuple_area(dimensions: (u32,  u32)) ->  u32{
    dimensions.0  *  dimensions.1
}

fn calc_area(rectangle:  &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
mod x {
    #[derive(Debug)]
    pub struct Point<T>{
        pub x: T,
        y: i32
    }

    impl<T>  Point<T> {
        pub fn new(val: T) -> Self {
            Point {
                x:  val,
                y:45
            }
        }

        pub fn get_y(&self) -> i32{
            self.y
        }
    }

}


pub enum  Color<T> {
    R(T),
    G(i32),
    B(i32)
}



fn main()  {
    let v = vec![1,2,3,4];

    largest(&v);

    println!("{:#?}",  crate::x::Point::new("hello rust!".to_string()));
    println!("value of y: {}", x::Point::new("xyz").get_y() );

    if let Color::R(val) =  Color::R("very red") {
        println!("color is {}",  val);
    }

}


fn largest<T: std::cmp::PartialOrd> (list:  &[T]) -> &T {
    let mut max =  &list[0];

    for i in list{
        if i >  max{
            max =  i;
        }
    }

    return  max;
}
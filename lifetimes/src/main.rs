
fn main() {
    let my_str =  String::from("hello new string here");
    let cmp_string =  "hey another binary string";

    {
        let mut gory =  String::new();
        gory.push_str("hey this is a gory image");

        let y =  longest(my_str.as_str(), gory.as_str());
        println!("inner scope {}",  y);
        {
            println!("double inner scope {}", longest(my_str.as_str(), y));
        }
    }


    println!("{}",  longest(my_str.as_str(), cmp_string));

}



fn longest<'a>(x:&'a str,  y:&'a str) -> &'a str {
    if x.len() >  y.len() {
        return x ;
    }

    y
}


trait  X {
    fn some_kind (item:&str) -> String;
}

struct ImplPart<'a, T>{
    x:&'a T
}

impl<'a,T>  ImplPart<'a, T> {
    
}
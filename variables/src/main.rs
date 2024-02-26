fn main() {
    // let mut x = 5;
    // println!("value of x: {x}");
    // x = 6;
    // println!("value of x: {x}");

    let x =  5;
    
    println!("this is x: {x}");

    let x = 5 + 1;
    {
        let x =  x + 1;
        println!("the value of x:  {x}");
    }


    println!("the value of x:  {x}");
}

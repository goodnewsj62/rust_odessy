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


    let tup =  (254, 9.1,  6);

    let (x, _y, _z )=  tup;

    let mut f =  tup.1;

    println!("f: {f}");

    f =  88.9;


    println!("f: {f}");

    println!("the value of x:  {x}");


}

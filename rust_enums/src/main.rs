#[derive(Debug)]
struct IPV4;

#[derive(Debug)]
struct IPV6;

#[derive(Debug)]
enum IPAddrs{
    V4(IPV4),
    V6(IPV6)
}

// impl IPAddrs {
//     fn end(&self){
//         //
//     }
// }



#[derive(Debug)]
enum USstates{
    Alabama,
    Chicago,
    Newyork
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USstates),
}


fn value_in_cents(coin:Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime =>10,
        Coin::Quarter(state) => {
            println!("state on the coin is {:?}", state);
            25
        }
    }
}

fn catch_all(arg: u32){
    match arg{
        1 => println!("hello one here"),
        4 =>  println!("hello four here"),
        9 => println!("hello 9ine here"),
        _ => println!("i am caught all others")
}

}

fn main() {
    let ipvaddr =  IPAddrs::V4(IPV4{});
    println!("Hello enums {:?}",  ipvaddr );

    println!("value of dime in cents {}", value_in_cents(Coin::Dime) );

    println!("value of quarter in cents {}",  value_in_cents(Coin::Quarter(USstates::Alabama)));

    let some_val =  Some(56);
    let another_val:Option<u32> =  None;

    let six =  plus_one(some_val);
    let none =  plus_one(another_val);

    catch_all(2);

    let coin =  Coin::Quarter(USstates::Chicago);

    if let Coin::Quarter(val) =  coin {
        println!("the value in the if let {:?}", val);
    }else{
        println!("other coins not quater");
    }

}



fn plus_one(value:  Option<u32>)-> Option<u32>{
    match value {
        Some(val)=> {
            Some(val +  1)
        }
        None =>  None

    }
}
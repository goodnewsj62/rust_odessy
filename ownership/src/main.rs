fn main() {
    let s1 =  String::from("some value");
    
    take_ownership(s1);

    let x =  4;
    make_copy(x);

    let mut s2 =  String::from("some value");
    change(&mut s2);


    ref_practice();
    first_word_prog(&s2);

    println!("{}",  &s2[1..6])

}



fn take_ownership(str:String){
    println!("the value gotten: {str}");
}

fn make_copy(some_int: i32) {
    println!("some_int {} value",  some_int);
}

fn change(some_string:  &mut String) {
    some_string.push_str("hello world");
}

fn ref_practice(){
    let mut s =  String::from("hello");

    let s1 =  &s;
    let s2 =  &s;

    println!("s1{} and s2{}",  s1, s2);
    // always remember a reference scope ends where it is last used

    let s3 =  &mut s;

    println!("{}", s3);

    // printing or using s1 or s2 here will cause the 
    //code to panic because a mut s cannot happen as s1 and s2 
    //doesn't expect s to change

}

fn first_word_prog(s:&String) -> &str{
    let bytes =  s.as_bytes();

    for (i , &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
use std::collections::HashMap;



fn main() {
    let mut v: Vec<i32> =  Vec::new();

    v.push(5);
    v.push(8);

    let v =  vec![1,2,3];

    let third:  &i32 =  &v[2];

    println!("the thrid value stored {}",  third);

    for n in 0..3 {
        println!("for val{}", &v[n])
    }

    let third:  Option<&i32> =  v.get(2);

    match third{
        Some(third) => println!("the bind value {}",  third),
        None =>  println!(" no value given")
    }


    let mut v =  vec![100,32,57];

    for i in &mut v {
        *i +=  50;
    }


    let s =  "some nice food";

    let str_  =  s.to_string();

    println!("value of str_: {str_}");

    let str_  =  String::from("ate some nice food!");

    println!("what is in me: {str_}");

    let mut ant =  String::from("i rep the ant world");
    ant.push_str("! hello ants");

    println!("calling the ants {ant}");

    ant.push('*');

    println!("{ant}");

    let new_s = format!("{str_}-{ant}");


    println!("{new_s}");


    let mut scores =  HashMap::new();

    scores.insert(String::from("gryphindor"), 50);
    scores.insert(String::from("copper"), 80);

    let to_get =  String::from("copper");
    let score =  scores.get(&to_get).copied().unwrap_or(0);

    println!("the score of copper {}",  score);

    for (key, value) in &scores {
        println!("{key} - {value}");
    }


    let string_spaces =  "hey i love creating stuff";

    let mut map =  HashMap::new();

    for val in  string_spaces.split_ascii_whitespace(){
        let value_gotten =  map.entry(val).or_insert(0);
        *value_gotten += 1;
    }

    println!("{:#?}",  map);


}

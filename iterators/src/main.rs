fn main() {
    let v1 =  vec![1,2,3,4,5,6];

    let v1_iter =  v1.iter();

    for n in v1_iter {
        println!("{}",  n);
    }

    // consumer adapters

    let v2_iter =  v1.iter();
    let total: i32 =  v2_iter.sum();

    println!("\n\n{}",  total);
    
    let v3_iter =  v1.iter();

    //iterator adapters
    let v3:Vec<_> =  v3_iter.map(|f|f + 2).collect();

    println!("{:?}",  v3);


}

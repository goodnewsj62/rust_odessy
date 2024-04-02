fn main() {
    let add_one_v3 =  |num|  num + 1;

    let x = add_one_v3(1);

    println!("printing the value of closure {}",  x);

    let mut list =  vec![1,2,3,4];

    let list2 =  &list;

    println!("hey list2 {:?}",  list2);

    println!("before defining closure {:?}",  list);

    let mut borrow_mutably  =  || list.push(1);

    borrow_mutably();

    println!("after mutable borrow {:#?}",  list)
}
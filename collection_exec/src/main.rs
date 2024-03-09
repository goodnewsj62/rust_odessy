use std::collections::HashMap;

fn main() {
    let array =  [1,5,4,8,6,8];
    let mut vec_of_num: Vec<i32> =  Vec::new();

    for num in array{
        vec_of_num.push(num);
    }


    println!("{:?}",  vec_of_num);

    sort_vector_number(&mut vec_of_num);


    println!("{:?}",  vec_of_num);

    println!("median {}", get_median(&vec_of_num));


    println!("mode {}",  get_mode(&vec_of_num));

}

fn sort_vector_number(list_of_num:  &mut Vec<i32>){

    let size =  list_of_num.len();

    for i in 0..size {
        let nested_start =  i  +  1;

        if nested_start == size{
            break;
        }

        for j in nested_start..size {
            if list_of_num[i] >  list_of_num[j]{
                let hold_i =  list_of_num[i];
                let hold_j = list_of_num[j];

                let i_ref =  &mut list_of_num[i];

                *i_ref = hold_j;

                let   j_ref =  &mut list_of_num[j];
                *j_ref =  hold_i;
    
            }
        }
    }
}


fn get_median(vector: &Vec<i32>) ->  i32{
    let size =  vector.len();
    if size == 0{
        return 0;
    }

    let mid =  (size / 2) as usize;

    vector[mid]
}

fn get_mode(vector:  &Vec<i32>) -> i32{
    let mut map =  HashMap::new();

    for i in vector{
        let value =  map.entry(i).or_insert(0);
        *value += 1;

    }

    let mut max =  (0,0);

    for (key,  value) in map{
        if value >  max.1{
            max = (*key, value);
        }
    }

    max.0
}
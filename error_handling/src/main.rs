use std::{fs::File, io::{self, Read}};

fn main() {
    // just_panic();
    // system_panic();

    let op_result =  File::open("file.txt");

    let file =  match op_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("file.txt") {
                Ok(val)=> val,
                Err(e)=>panic!("could not create file  {:#?}",  e)
            }
            error => panic!("an error occurred  {}",  error)
        }
    };


    let file_2 =  File::open("file.txt").expect("the file was not found in the file system");


    if let Ok(val) =  get_string_from_file() {
        println!("{val}");
    }else{
        panic!("oops could not get the value");
    }

}


fn just_panic() {
    panic!("i am about to throw up!");
}


fn system_panic() {
    let my_vec =  vec![1,2,4,5];

    my_vec[99];
}

fn get_string_from_file() -> Result<String,  io::Error>{
    let mut str_val =  String::new();
    File::open("file.txt")?.read_to_string(&mut str_val)?;

    Ok(str_val)
}
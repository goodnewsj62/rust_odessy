use std::{self, env,process};
use minigrep;

fn main() {
    let args:  Vec<String> =  env::args().collect();




    let config =  minigrep::Config::build(&args).unwrap_or_else(|err|{
        println!("oops! {}",  err);
        process::exit(1);
    });
    println!("string_query: {}  file_name {}",   config.query, config.file_path);

    

    if let Err(err) =  minigrep::run(config) {
        println!("Application error: {}",  err);
        process::exit(1);
    }

}


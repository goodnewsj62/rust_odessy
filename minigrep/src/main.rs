use std::{self, env,process};
use minigrep;

fn main() {
    let args:  Vec<String> =  env::args().collect();
    let ignore_case =  env::var("IGNORE_CASE").is_ok();

    let config =  minigrep::Config::build(&args, ignore_case).unwrap_or_else(|err|{
        println!("oops! {}",  err);
        process::exit(1);
    });


    if let Err(err) =  minigrep::run(config) {
        println!("Application error: {}",  err);
        process::exit(1);
    }

}


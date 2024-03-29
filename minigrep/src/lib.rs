use std::{self, error, fs};


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build (args:  &Vec<String>, ignore_case:  bool) -> Result<Self,  &'static str> {
        if args.len() < 3 {
            return  Err("not enough arguments");
        }
        Ok( Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case
        })
    
    }
}



pub fn run(config:Config) -> Result<(), Box<dyn error::Error>>{
    let content =  fs::read_to_string(config.file_path)?;
    let result =  if config.ignore_case {search(config.query.as_str(), content.as_str())} else {case_insensitive_search(config.query.as_str(), content.as_str())};

    for line in result{
        println!("{}", line);
    }

    
    Ok(())
}


pub fn search<'a>(query:&str, content:&'a str) -> Vec<&'a str > {
    let mut found =  Vec::new();

    for line in content.lines() {
        if line.contains(query){
            found.push(line.trim());
        }
    }
    
    found
}

pub fn case_insensitive_search<'a>(query: &str,  content:&'a str) -> Vec<&'a str> {
    let query  =  query.to_lowercase();
    let mut result =  Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query){
            result.push(line.trim());
        }
    }
    
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, contents)
        );
    }
}
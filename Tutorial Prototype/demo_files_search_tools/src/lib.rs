use std::{fs, error::Error, env};


pub struct Config{

    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,

}impl Config {

    pub fn build(args: &[String]) -> Result<Config,&'static str>{

        if args.len()<3{
            // panic!("not engough arguments");
            return Err("not enough arguments");
        }


        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok(); //is_ok 为是否有值

        Ok((Config{ query, file_path, ignore_case, }))

    }
}

// impl Config {
//     pub fn build(v:&[String]) -> Result<Config, &'static str> {
//         if v.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = v[1].clone();
//         let file_path = v[2].clone();

//         let ignore_case_flag = env::var("IGNORE_CASE").ok();
//         let ignore_case = match ignore_case_flag.as_ref().map(String::as_ref) {
//             None => false,
//             Some("0") => false,
//             Some(_) => true
//         };

//         Ok(Config { query, file_path, ignore_case })
//     }
// }








pub fn run (config: Config ) -> Result<(),Box<dyn Error>>{

    // let config = fs::read_to_string(config.file_path).expect("should have been able to read the file path ");
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }


    // println!("with text: \n\n{} ",config);
    Ok(())
}





pub fn search_case_insensitive<'a>(query:&str , contents: &'a str, ) ->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}















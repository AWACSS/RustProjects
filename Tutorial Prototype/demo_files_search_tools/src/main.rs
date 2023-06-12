//入门实战：文件搜索工具


use std::env;
use std::{fs, error::Error, process};
use demogrep::Config;
use demogrep::run;



fn main() {
    //std::env::args()函数调用得到程序的入口参数，返回执行文件的路径，只使用一次
    let args: Vec<String> = env::args().collect(); 

    let config = Config::build(&args).unwrap_or_else(|err|
        {
            println!("problem parsing arguments: {err}");
            process::exit(1);
        }      
    );
   
    

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //读取文件
    //读取指定的文件内容,返回的 config 是 std::io::Result<String> 类型

    if let Err(e)=run(config){
        println!("Application error: {e}");
        process::exit(1)

    }

    
}


















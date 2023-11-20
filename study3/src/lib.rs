use std::{error::Error,fs};
use std::env;
use std::process;


pub struct Config{
    query:String,
    filename:String,
    pub case_sensitive:bool//切换是否忽略大小写
}
impl Config{
    fn new(args:&[String])->Result<Config,&'static str>{
        if args.len()<3{
            return Err("参数不够");
        }
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{
            query:args[1].clone(),
            filename:args[2].clone(),
            case_sensitive,//不区分大小写
        })
    }
}
//返回包含query的vec,区别大小写
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
// 返回包含query的vec,不区分大小写
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    let query= query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
//打印
fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.filename)?;//传播错误
    let results=if config.case_sensitive{
        search(&config.query,&contents)
    }else {
        search_case_insensitive(&config.query,&contents)
    };
    for line in results{
        println!("{}",line)
    }
    Ok(())
}

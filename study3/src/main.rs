use std::{error::Error,fs};
use std::env;
use std::process;
// >  文件名
// 重定向 : 告诉终端将打印信息输出到指定文件而不是终端上面
// eprintln!("{}",err);
// eprintln!宏用来向标准错误打印信息

pub struct Config{
    query:String,
    filename:String,
}
impl Config{
    fn new(args:&[String])->Result<Config,&'static str>{
        if args.len()<3{
            return Err("参数不够");
        }
        Ok(Config{
            query:args[1].clone(),
            filename:args[2].clone(),
        })
    }
}
//返回包含query的vec
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
//打印
fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.filename)?;//传播错误
    for line in search(&config.query,&contents){
        eprintln!("{}",line);
    }
    Ok(())
}
// cargo run 1 Cargo.toml
fn main() {
    // collect() 方法将迭代器转换为一个集合例如： Vec。
    let args:Vec<String>=env::args().collect();
    let config=Config::new(&args).unwrap_or_else(|err|{
        eprintln!("err={}",err);
        process::exit(1);
    });
    if let Err(e)=run(config){
        eprintln!("Application error:{}",e);
        process::exit(1);
    }
}

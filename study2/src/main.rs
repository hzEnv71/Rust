use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, format, Formatter};
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use crate::Coin::{Penny, Quarter};
use crate::UsState::{Alabama, Alaska};


fn main() {

    let mut x = 5;
    println!("x={x}");
    x = 6;
    println!("x={x}");
    const N: u32 = 60;
    println!("N={}", N);

    let y = 6;
    let y = y + 1;
    {
        let y = y * 2;
        println!("inner y={y}");
    }
    println!("outer y={y}");

    let spaces = "   ";
    let spaces = spaces.len();

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let xxx: (i32, f64, u8) = (500, 6.4, 1);
    let one = xxx.0;
    let two = xxx.1;
    let three = xxx.2;
    println!("one={one},two={two},three={three}");

    let a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let __a = [3; 5]; //默认值，长度
    println!("a[0]={}", a[0]);
    println!("_a[0]={}", _a[0]);
    println!("__a[0]={}", __a[0]);

    println!("sum={}", sum(7, 8));

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    for element in a {
        print!("{element}  ");
    }
    println!();
    for number in (1..6).rev(){
        print!("{number}  ");
    }
    println!();

    //copy 特征//基本类型都实现了
    // 需要分配内存的需要clone;
    let s1=String::from("hello");
    // let s2=s1;
    let s2=s1.clone();
    println!("s1={},s2={}",s1,s2);

    let num1=666;
    let num2=num1;
    println!("num1={},num2={}",num1,num2);

    let s1=String::from("qwe");
    // let  s2=copy_str(s1.clone());
    let s2= copy_str_1(&s1);
    println!("s1={},s2={}",s1,s2);

    let mut s=String::from("rust");
    push_back(&mut s);
    println!("s={s}");
    let copy_1=&s;
    let copy_2=&s;
    // let copy_3=&mut s;//错误
    println!("{} and {}",copy_1,copy_2);
    let copy_3=&mut s;
    println!("{}",copy_3);

    let mut s=String::from("hello world");
    let word=first_word(&s);
    println!("{word}");
    s.clear();


    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];


    let mut s = String::from("hello world");

    let word = first_word_1(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
    //切片
    let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];
    let slice=int_slice(&a[1..4]);
    println!("{:?}", slice.iter());

    //结构体
    let mut user1 =User{
        id:124321,
        username:String::from("钟馗"),
        // username:"钟馗",
        status:true,
    };
    user1.id=34253;
    println!("{:?}",user1);
    // 使用结构更新语法
    let user2=User{
        id :3254235,
        ..user1
    };
    println!("{:?}",user2);
    let parm=(3,5);
    let ans=(parm);
    println!("ans={:?}",ans);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    //方法
    println!("area={}",rect1.area());

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}",Rectangle::square(30,50).area());


    // let home = IpAddr::V4(String::from("127.0.0.1"));
    //
    // let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let some_number=Some(5);
    let some_char=Some('q');
    let absent_number:Option<i32>=None;
    let x:i8=5;
    let y:Option<i8>=Some(5);
    // let sum=x+y;//error

    let coin=value_in_cents(Coin::Quarter(Alabama));
    println!("coin={}",coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), //通配
    }

    let mut count = 0;
    let coin=Quarter(Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    //if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin=Quarter(Alaska);
    let mut count = 0;
    if let Coin::Quarter(status) = coin {
        println!("State quarter from {:?}!",status);
    } else {
        count += 1;
    }

    //Vector
    let v:Vec<i32>=Vec::new();
    let mut v =vec![1, 2, 3];
    v.push(7);
    println!("{:?}",v);

    //get()和 [index]
    // let value=&v[100];//panic
    // println!("{} {}", value, v[100]);//panic

    let value:Option<&i32>=v.get(100);
    match value {
        Some(val)=>println!("val={}",val),
        None=>println!("no element"),
    }
    //由于向量将值放在内存中相邻，如果在向量的末尾添加新元素可能需要分配新内存并将旧元素复制到新空间，如果没有足够的空间将所有元素放在当前存储向量的相邻位置
    // let val=&v[0];
    // v.push(9);
    // println!("val={}",val);

    //迭代
    let mut v=vec![12,34,28];
    for i in &mut v{
        *i+=20;
        println!("{i}");
    }
    for (i,val) in v.iter().enumerate(){
        println!("{}..{}",i,val);
    }

    //String //UTF-8 ,不允许[index]下标访问
    let mut s=String::new();
    let mut text="hello world wonderful world";
    println!("{:?}",text.split_whitespace());

    let mut s1="34532545";
    let s2=s1.to_string();
    s.push_str(s1);
    println!("{},{},{}",s,s1,s2);

    //+运算符使用add方法
    // fn add(self, s: &str) -> String {}
    let s1=String::from("he");
    let s2="llo".to_string();
    let s3=s1+ &s2;
    println!("s3={}",s3);
    //format!
    let s1=String::from("he");
    let s3="_world".to_string();
    let s4=format!("{s1}{s2}{s3}");
    println!("s4={}",s4);
    //chars;
    let hello = "Здравствуйте";
    println!("len(hello)={}",hello.len());
    let s = &hello[0..4];
    for c in hello.chars(){
        print!("{}",c);
    }
    println!("\n=============");
    //bytes;
    for b in hello.bytes(){
        print!("{} ",b);
    }
    println!("\n=============");
    //HashMap
    //需要 use std::collections::HashMap;(没有加入预处理)
    use std::collections::HashMap;
    let mut map=HashMap::new();
    map.insert(String::from("hello"),10);
    let val=map.get(&String::from("hello")).copied().unwrap_or(0);
    println!("val={}",val);
    for (key,val) in &map{
        println!("{key}:{val}");
    }
    map.insert(String::from("hello"),30);//更新会直接覆盖
    map.entry(String::from("hello")).or_insert(50);//entry方法的返回值是一个名为Entry的枚举，表示可能存在或不存在的值
    println!("{:?}",map);
    let tmp:Entry<String,i32>=map.entry(String::from("hello"));
    println!("tmp={:?}",tmp);
    *(map.entry(String::from("hello")).or_insert(55))+=1;//根据旧值进行更新
    println!("{:?}",map);

    //zip
    let key=[String::from("hello"),String::from("world")];
    let val=[3,5];
    let mut map:HashMap<_,_>=key.iter().zip(val.iter()).collect();
    let k=&String::from("rust");
    map.insert(k, &6);
    println!("{:?}",map);

    //panic
    // panic!("crash and burn");
    //RUST_BACKTRACE=full

    use std::fs::File;
    let g=File::open("a.txt");
    let g_file=match g {
        Ok(file)=>file,
        // Err(error)=>panic!("error={}",error),
        Err(error)=>match error.kind() {
            ErrorKind::NotFound=>match File::create("a.txt") {
                Ok(fc)=>fc,
                Err(err)=>panic!("err={}",err),
            },
            other_err=>{
                panic!("other_err={}",other_err);
            }
        }
    };
    //标准库文档中查找unwrap_or_else方法
    let g_file=File::open("a.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("a.txt").unwrap_or_else(|error|{
                panic!("error={}",error);
            })
        }else {
            panic!("error={}", error);
        }
    });
    //unwrap
    let g_file=File::open("a.txt").unwrap();
    let g_file=File::open("a.txt").expect("a.txt should be included");

    //传播错误

    //泛型
    println!("{},{}",f1("string"),f1("int"));

    //trait
    let user1=user{uname:"关羽".to_string(),pwd: "qwe".to_string(),age:23};
    println!("{}",user1.f1());
    user1.f2();

    trait作为参数(&user1);
    trait约束(user1);
    println!("trait作为返回值{}",trait作为返回值().f1());

    //'static 它表示整个程序的执行期。所有的字符串字面量都拥有这个静态生命周期


    //闭包
    //如果闭包只有一条语句，那么可以省略花括号
    //所有闭包都至少实现了 Fn 、 FnMut 、 FnOnce 中的一个 trait。//获取所有权、可变借用、不可变借用
    let sum= |num1,num2|num1+num2;
    println!("{:?}",sum(100, 200));
    let mut cache =Cacher::new(|a|a);
    println!("value(1):{}",cache.value(1));
    println!("value(2):{}",cache.value(2));

    let x=5;
    let f=|z|(z==x);
    println!("f={} {}", f(5), x);


    let x =vec![1,2,3];
    let equal_to_x = move |z| z == x;
    // println!("{:?}", x);//错误因为使用了move x 的所有权和值已经被移动到闭包中
    let y=vec![1,2,3];
    println!("{:?}", equal_to_x(y));

    //调用 sum 的过程中获取了 v1 的所有权,因此这个迭代器无法被后面的代码继续使用
    let v1:Vec<i32>=vec![5,3,2];
    println!("v1[0]={:?}",v1.iter().next());
    let mut v1_iter =v1.iter();
    let _sum:i32=v1_iter.clone().sum();
    println!("sum1={},sum2={}",v1.iter().sum::<i32>(),v1.iter().sum::<i32>());
    println!("_sum1={},_sum2={}",v1_iter.clone().sum::<i32>(),v1_iter.clone().sum::<i32>());


}
struct Cacher<T> where T:Fn(i32)->i32{
    cal:T,
    value:Option<i32>,
}
impl<T> Cacher<T> where T:Fn(i32)->i32{
    fn new(cal:T)->Cacher<T>{
        Cacher{
            cal,
            value:None,
        }
    }
    fn value(&mut self,arg:i32)->i32{
        match self.value {
            Some(v)=>v,
            None=>{
                let v=(self.cal)(arg);
                self.value=Some(v);
                v
            }
        }
    }
}
//结构体生命周期
struct  struct_lifetime<'b>{
    life:&'b str,
}
//生命周期
pub fn 生命周期<'a>(x:&'a str, y:&'a str) ->&'a str{
    if x.len()<y.len(){
        x
    }else{
        y
    }
}

pub fn trait作为返回值()-> impl Inter1 {
    user{
        uname:"刘备".to_string(),
        pwd:"ewr".to_string(),
        age:19,
    }
}
//trait约束
// pub fn trait约束<T:Inter1+Inter2>(item1:T){
// 使用 where 从句来简化 trait 约束
pub fn trait约束<T>(item1:T)where T:Inter1+Inter2 {
    println!("trait约束{}",item1.f1());
}
//使用 trait 作为参数
pub fn trait作为参数(item:&impl Inter1){
    print!("使用trait作为参数");
    item.f2();
}
//trait（特征）类似接口
pub trait  Inter1{
    // fn f1(&self)->String;
    fn f1(&self)->String{//若没有实现则使用默认实现
        format!("默认实现1")
    }
    fn f2(&self);
}
pub trait  Inter2{
    // fn f1(&self)->String;
    fn f3(&self)->String{//若没有实现则使用默认实现
        format!("默认实现2")
    }
    fn f4(&self);
}
pub struct user{
    pub uname:String,
    pub pwd:String,
    pub age:i32,
}
// impl Display for user{
//     fn fmt(&self, f: &mut Formatter<'_>) ->Result<(),std::fmt::Error>{
//
//     }
// }
//实现trait
impl Inter1 for user{
    // fn f1(&self) -> String {
    //    format!("f1={},{},{}",self.uname,self.pwd,self.age)
    // }

    fn f2(&self) {
        println!("f2={},{},{}",self.uname,self.pwd,self.age)
    }
}
impl Inter2 for user{
    // fn f3(&self) -> String {
    //    format!("f1={},{},{}",self.uname,self.pwd,self.age)
    // }

    fn f4(&self) {
        println!("f2={},{},{}",self.uname,self.pwd,self.age)
    }
}
//泛型结构体
struct Po<T,E>{
    x:T,
    y:E
}
// 结构体泛型和方法泛型可以不同
impl<TT,EE> Po<TT,EE>{
    fn x(&self)->&TT{
        &self.x
    }
    fn y(&self)->&EE{
        &self.y
    }
}
fn f1<T>(a:T)->T{a}
/*fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}*/
//找到给定文本中第一行的最后一个字符
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// use std::fs::File;
use std::io::{self,Read};
use std::thread::sleep;

//使用fs::read_to_string使其更短的方法
fn read_username_from_file2()->Result<String,io::Error>{
   fs::read_to_string("a.txt")
}
//?操作  //Result<T, E>
//函数的返回类型必须是Result，以便它与此return兼容
fn read_username_from_file1()->Result<String,io::Error>{
    let mut username=File::open("a.txt")?;
    let mut uname=String::new();
    username.read_to_string(&mut uname)?;
    Ok(uname)
}

//从文件中读取用户名   //Result<T, E>
fn read_username_from_file()->Result<String,io::Error>{
    let username=File::open("a.txt");
    let mut username_file=match username {
        Ok(file)=>file,
        Err(err)=>return Err(err),
    };
    let  mut uname=String::new();
    match username_file.read_to_string(&mut uname) {
        Ok(_)=>Ok(uname),
        Err(err)=>Err(err),
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),

    }
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(status) => {
            println!("State quarter from {:?}!", status);
            25
        },
    }
}

//Option<T>
// enum Option<T>{
//     None,
//     Some(T),
// }
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}



enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
//枚举
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
//
//方法
impl  Rectangle {
    fn area(&self) -> u32{
        self.width*self.height
    }
    // 不是方法的关联函数通常用于将返回结构新实例的构造函数
    fn square(width:u32, height:u32) -> Self {
        Self{
            width,
            height,
        }
    }
}
//debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用没有命名字段的元组结构来创建不同的类型
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 用元组重构
fn muti(p: (i32, i32)) ->i32{
   p.0* p.1
}
#[derive(Debug)]
struct User{
    id:u32,
    username:String,
    // username:&str,
    status:bool,
}
// 参数名称和结构字段名称完全相同
fn build_user(id:u32,username:String,status:bool)->User{
// fn build_user(id:u32,username:&str,status:bool)->User{
    User{
        id,
        username,
        status,
    }
}
fn int_slice(nums:&[i32])-> &[i32]{
    nums
}

fn first_word_1(s: &String) -> &str {//字符串切片
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn push_back(s:&mut String){
    s.push_str(" very good!");
}
fn copy_str_1(s:&String) ->&String{
    // s.push('a');
    s
}
fn copy_str(s:String) ->String{
    s
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
    // return a+b;
}

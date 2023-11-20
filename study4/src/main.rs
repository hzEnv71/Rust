use std::cell::{Ref, RefCell, RefMut};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use crate::List::{Cons, Nil};

#[derive(Debug)]
// enum  List{
//     Cons(i32,Box<List>),
//     Nil,
// }

// enum List{
//     Cons(i32,Rc<List>),
//     Nil,
// }

// enum  List{
//     Cons(Rc<RefCell<i32>>,Rc<List>),
//     Nil,
// }

enum  List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}

impl  List{
    fn tail(&self)->Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_,item)=>Some(item),
            Nil=>None
        }
    }
}
#[derive(Debug)]
struct  MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T>{
    type Target =T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    //智能指针
    // Box :在 heap 内存上分配值
    // Rc :启用多重所有权的引用计数类型
    // Ref 和 RefMut , 通过 RefCell 访问： 在运行时而不是编译时强制借用规则的类型

    //在编译的时候 ， rust 需要知道一个类型所占的空间大小。
    // let b=Box::new(5);
    // println!("b={}",b);
    // let list=Cons(4, Box::new(
    //             Cons(5,Box::new(
    //                 Cons(6,Box::new(Nil))))));
    // println!("list={:?}",list);

    let x = 5;
    let y = &x;
    println!("y={:?}", y);
    let y = MyBox::new(x);
    println!("y={:?}", *y);//*y 需要实现 Deref trait

    //函数和方法的隐式解引用转换
    //将某个特定类型的值引用作为参数传递给函数或方法，但传入的类型与参数类型不一致时，解引用转换就会自动发生

    drop(y);//可以调用的是 std::mem:drop 函数
    let drop1 = MyBox(6);
    println!("_________drop___________:{}", drop1.0);//变量的丢弃顺序与创建顺序相反。


    // let a =Cons(5,Box::new(Cons(10,Box::new(Nil))));
    // let b=Cons(3,Box::new(a));
    // let c=Cons(4,Box::new(a));//报错原因：Box<T> 无法让多个列表同时持有另一个列表的所有权

    //解决方法：将 List 中 Box 改为 Rc
    //克隆 Rc 会增加引用计数
    // let a=Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Cons(8,Rc::new(Nil)))))));
    // println!("count_a={}",Rc::strong_count(&a));
    // let b=Cons(3,Rc::clone(&a));
    // println!("count_ab={}",Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count_abc={}",Rc::strong_count(&a));
    // }//c生命周期完了
    // println!("count_={}",Rc::strong_count(&a));
    // println!("count_week={}",Rc::weak_count(&a));

    // RefCell 和内部可变性模式
    // 内部可变性模式在它的数据结构中使用了 unsafe （不安全）代码来绕过 rust 正常的可变性和借用规则
    // Rc 和 RefCell 只能被用于单线程场景中。

    // Rc 允许一份数据有多个所有者，而 Box 和 RefCell 都只能有一个所有者
    // Box 允许在编译时检查的可变或不可变借用， Rc 仅允许编译时检查的不可变借用，RefCell 允许运行时检查的可变或不可变借用
    // 由于 RefCell 允许我们在运行时检查可变引用，所以即便 RefCell 本身是不可变的，我们仍然能够更改其中存储的值

    // let value=Rc::new(RefCell::new(7));
    // let a=Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
    // let b=Cons(Rc::new(RefCell::new(2)),Rc::clone(&a));
    // let c=Cons(Rc::new(RefCell::new(3)),Rc::clone(&a));
    // *value.borrow_mut()+=20;
    //
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);
    // 循环引用会造成内存泄漏

    let x = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("x initial rc count ={}", Rc::strong_count(&x));
    println!("x next item = {:?}", x.tail());
    let y = Rc::new(Cons(7, RefCell::new(Rc::clone(&x))));
    println!("x rc count after y creation ={}", Rc::strong_count(&x));

    if let Some(link) = x.tail() {
        *link.borrow_mut() = Rc::clone(&y);
    }

    println!("y rc coutn after changing x = {}", Rc::strong_count(&y));
    println!("x rc count after changing y = {}", Rc::strong_count(&x));
    println!("================================================");

    // 下面这行，会造成循环引用造成栈溢出
    // println!("a next item ={:?} ", x.tail());

    //使用 Weak 代替 Rc 来避免循环引用


    let ch = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "ch strong ={} , weak = {}",
        Rc::strong_count(&ch),
        Rc::weak_count(&ch),
    );


    {
        println!("ch parent={:?}", ch.parent.borrow().upgrade());
        let pa = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&ch)]),
        });
        *ch.parent.borrow_mut() = Rc::downgrade(&pa);
        println!(
            "pa strong ={}, weak = {}",
            Rc::strong_count(&pa),
            Rc::weak_count(&pa),
        );

        println!(
            "ch strong = {} , weak = {}",
            Rc::strong_count(&ch),
            Rc::weak_count(&ch),
        );
    }

    println!("pa child={:?}", ch.parent.borrow().upgrade());
    println!(
        "ch strong = {} , weak = {}",
        Rc::strong_count(&ch),
        Rc::weak_count(&ch),
    );


    //无畏并发
    //创建线程
    let thread1 = thread::spawn(|| {
        println!("thread1启动..");
        for x in 1..10 {
            println!("thead1________x={}", x);
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("main thread 启动..");
    for x in 1..=5 {
        println!("main_________x={}", x);
        thread::sleep(Duration::from_millis(1));
    }
    //使用 join 句柄等待所有线程结束
    thread1.join().unwrap();//阻塞线程


    //move
    let v1 = vec![1, 2, 3];
    let thread1 = thread::spawn(move || println!("v1_{:?}", v1));//move 会强制闭包获得它所需值的所有权
    // drop(v1);////在这里已经清理了 v 但新线程却还需要使用
    thread1.join().unwrap();


    //通道
    use std::sync::mpsc;
    let (tz, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tz);


    thread::spawn(move || tz.send("你好哦".to_string()).unwrap());//消息已经送了，无法再使用它
    // recv() 方法会阻塞主线程的执行直到有值被传入通道
    // try_recv () 方法不会阻塞线程，它会立即返回 Result
    // let reveiced=rx.try_recv().unwrap();
    let reveiced = rx.recv().unwrap();
    thread::spawn(move || tx1.send("你好哦___".to_string()).unwrap());//消息已经送了，无法再使用它
    println!("接受的消息={}", reveiced);

    // 通过共享内存来通信。
    // 多个线程可以同时访问相同的内存地址。
    //Mutex 是一种智能指针，对 lock 的调用会返回一个名为 MutexGuard 的智能指针。

    ////单线程环境下
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 6;
    }
    println!("m={:?}", m);
    //多线程环境下
    // Arc它拥有类似于 Rc 的行为，又保证了自己可以被安全地用于并发场景。 A 代表原子（atomic）表明自己是一个原子引用计数
    let counter=Arc::new(Mutex::new(10));
    let mut v1=vec![];

    for _ in 1..5{
        let counter=Arc::clone(&counter);
        let tmp=thread::spawn(move || {
            let mut num=counter.lock().unwrap();
            *num+=1;
            println!("num={}", *num);
        });
        v1.push(tmp);
    }
    for x in v1{
        x.join().unwrap();
    }
    println!("counter lock={}",*counter.lock().unwrap());

    // 允许线程间转移所有权的 Send trait
    // 允许多线程同时访问的 Sync trait

    //模式匹配

    //高级特性
    //unsafe
    // 解引用祼指针
    // 调用不安全的函数或方法
    // 访问或修改可变的静态变量
    // 实现不安全 Rust

    let mut x=8;
    let r1=&x as *const i32;
    let r2=&mut x as *mut i32;

    unsafe {
        println!("r1={}",*r1);
        *r2+=9;
        println!("r2={}",*r2);
    }
    let address =0x012345usize;
    let r= address as *const i32;
    // unsafe { println!("r={}",*r);}
    unsafe { unsafe_f(); }
    // Rust 使用 extern 关键字简化创建和使用外部函数接口（FFI）的过程，FFI 是编程语言定义函数的一种方式，它允许其他外部的编译语言来调用这些函数
    // 任何在 extern 块中声明的函数都是不安全的
    extern "C"{
        #[no_mangle]//避免 Rust 在编译时改变它的 名称。
        fn abs(input:i32)->i32;
    }
    //访问或修改一个可变静态变量
    static  HELLO:&str="hello world";
    static  mut COUNT: i32 =0;
    fn add_to_count(x:i32){
        unsafe {
            COUNT+=x;
        }
    }
    add_to_count(5);
    unsafe { println!("COUNT:{}", COUNT); }


    //高级trait
    // 关联类型是 trait 中的类型占位符，它被用于 triat 的方法签名中


    ///默认泛型参数和运算符重载
    let p1=Point1{ x:3, y: 9 };
    let p2=Point1{x:7,y:1};
    let p_sum=p1+p2;
    println!("p_sum={:?}",p_sum);
    ///完全限定语法
    //<Type as Trait>::function(receiver_if_method, next_arg,...);
    //println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let x=Meters(7);
    let y=Millimeters(8);
    let xy=y.add(x);
    println!("xy={:?}",xy);

    ///类型别名
    type int =i32;
    //Rust 有一个名为 ！ 的特殊类型，它在类型系统中称为空类型，因为它没有任何值。它也从不返回的函数中充当返回值的类型
    // 使用了 never 的类型：（一些）
    // panic!
    // 循环 loop（它的返回值类型也是 ！）
    // continue 的返回值类型也是 !



    ///动态大小类型和 Sized trait

    // let s1:str="hello";//str 类型只有在运行时才能确定大小，因此我们不能创建出 str 类型的变量
    // let s2:str="world";
    //上面两个都报错，无法使用

    //上面代码换成  &str就可以使用了
    //因为 &str 实际上就由 str 的地址和它的长度两个值构成的
    // 在默认情况下，泛型函数只能被用于在编译时已经知道大小的类型。


    ///高级函数与闭包
    //函数指针实现了全部 3 种闭包 trait : Fn FnMut FnOnce
    // 与闭包不同， fn 是一个类型而不是一个 trait ，所以我们可以直接指定 fn 为参数类型，不用声明一个以 Fn trait 为约束的泛型参数与闭包不同， fn 是一个类型而不是一个 trait ，所以我们可以直接指定 fn 为参数类型，不用声明一个以 Fn trait 为约束的泛型参数
    println!("result={}",do_twice(add_one,1));


}


// 返回闭包
// 无法在函数直接返回一个闭包
// Rust无法推断出需要多大的空间来存储返回的闭包。
// 📖：解决： 使用 trait 对象：
fn returns_closure() -> Box<dyn Fn(i32) -> i32>{
    Box::new(|x| x+1)
}

fn add_one(x:i32)->i32 { x + 1 }
fn do_twice(f:fn(i32)->i32,arg:i32)->i32{f(arg)+f(arg)}

//使用 newtype 模式实现类型安全与抽象

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
       Millimeters(self.0+other.0*1000)
    }
}


//    //默认泛型参数和运算符重载
use std::ops::Add;
use std::process::Output;
use std::thread::sleep;

#[derive(Debug)]
struct Point1{x:i32,y:i32}
impl Add for Point1{
    type Output = Point1;

    fn add(self, other:Point1) -> Self::Output {
        Point1{
            x:self.x+other.x,
            y:self.y+other.y,
        }
    }
}

// 使用泛型在每次实现该 trait 的过程中标注类型，我们可以实现任意的迭代类型，从而使用得可以有多个不同版本的实现。我们可以为一个类型同时多次实现 trait
// 关联类型不需要在使用 trait 的方法时标注类型，不能为单个类型多次实现这样的 trait
struct Point{x:u32,y:u32}

impl Iterator for Point{
    type Item = u32;// // Item 是一个占位符，Iterator trait 的实现者需要为 Item 指定具体的类型
    fn next(&mut self) -> Option<Self::Item> {
       Some(7)
    }
}
//泛型版本
// pub trait Iterator<T> {
//     fn next (&mut self) -> Option<T>;
// }
//
// impl<T> Iterator<T> for Point {
//     fn next(&mut self) -> Option<T> {
//         Some(7)
//     }
// }


unsafe  trait  Foo{}
unsafe  fn unsafe_f(){
    println!("不安全函数。。")
}
#[derive(Debug)]
struct Node{
    value :i32,
    parent:RefCell<Weak<Node>>,
    children:RefCell<Vec<Rc<Node>>>////Rc 和 RefCell 都只能用于单线程中。
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {//不允许我们手动调用 drop 函数
        println!("最后执行_drop");
    }
}






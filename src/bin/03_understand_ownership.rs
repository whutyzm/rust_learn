/*
所有权原则
1.Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
2.一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
3.当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
*/

fn main() {
    let str_1 = String::from("hello");
    //let str_2 = str_1; //所有权被转移到了str_2，所以str_1失效
    let str_2 = str_1.clone();
    println!("{},world, str_2={}", str_1,str_2);

    let s = String::from("hahah");
    takes_ownership(s); //s的值移动到了函数里面
    //println!("{},报错",s);  //s在这里失效了

    let x = 5;
    makes_copy(x); //i32具有copy特征，所以后面可以继续使用
    println!("x = {}",x);

    //函数的返回值也可以转移所有权
    let s1 = gives_ownership();
    println!("s1={}",s1);

    let s2 = String::from("sss");
    let s3 = takes_and_gives_back(s2);
    //println!("s2={}",s2); 
    println!("s3={}",s3);

    //rust可以使用元组来返回多个值
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("'{}' 的长度是： {}", s2, len);

    //通过 &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它。
    //因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃。
    let s2 = String::from("aaa");
    let len = calculate_length_only(&s2);
    println!("'{}' 的长度是： {}", s2, len);

    //可变引用
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("change:s1={}",s1);

    //可变引用同时只能存在一个
    //可变引用与不可变引用不能同时存在
    {
        let mut s = String::from("hhh");
        let r1 = &s;
        let r2 = &s;
        println!("r1={},r2={}",r1, r2);
        let r3 = &mut s;
        //println!("r1={},r2={},r3={}",r1, r2,r3);
        println!("r3={}",r3);
        //let r4 = &mut s;
        //println!("r3={},r4={}",r3, r4);
    }
    

    
}

fn takes_ownership(some_str:String) {
    println!("some_str={},离开函数就被回收了",some_str);
}

fn makes_copy(some_integer:i32) {
    println!("some_integer={}",some_integer);
}

fn gives_ownership() -> String {
    let some_str = String::from("yangzm");
    some_str
}

fn takes_and_gives_back(some_str:String) -> String {
    some_str
}

fn calculate_length(some_string:String) -> (String,usize) {
    let length = some_string.len();
    (some_string, length)
}

fn calculate_length_only(some_str:&String) -> usize {
    some_str.len()
} //这里，some_str 离开了作用域。但因为它并不拥有引用值的所有权，所以some_str不会drop

fn change(some_str:&mut String) {
    some_str.push_str(",world,hhhhh");
}
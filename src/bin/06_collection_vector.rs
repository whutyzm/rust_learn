fn main() {
    //创建一个vector
    let _v1:Vec<i32> = Vec::new();
    let _v2 = vec![1,2,3];

    //push、pop
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    
    //读取vector元素
    let v = vec![1,2,3,4,5];

    let third = &v[2];
    println!("third,{}",third);

    let third = v.get(2);
    match third {
        Some(third)=> println!("third element is:{}",third),
        None=>print!("None"),
    }

    //index out of bounds
    //let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(item)=>println!("item:{}",item),
        None=>println!("index out of bounds"),
    }

    //遍历
    for item in &v {
        println!("item:{}",item);
    }

    let mut v = vec![1,2,3];
    for item in &mut v {
        *item += 10;
    }
    println!("vec:{:?}",v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
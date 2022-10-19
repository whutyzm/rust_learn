enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit=> println!("quit"),
            Message::Move{x,y}=> println!("Move,x={},y={}",x,y),
            Message::Write(x)=> println!("Write:{}",x),
            Message::ChangeColor(r,g,b)=> println!("ChangeColor:r={},g={},b={}",r,g,b),
        }
    }
}

fn main() {
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write_message = Message::Write(String::from("write!!!"));
    let change_color_message = Message::ChangeColor(12, 23, 34);
    quit_message.call();
    move_message.call();
    write_message.call();
    change_color_message.call();

    let num_five = Some(5);
    match num_five {
        Some(num) => println!("num:{}",num),
        None => println!("None"),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        other => println!("{}",other),
    }

    //_必须在最后
    let dice_roll = 9;
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        _ => println!("如果不需要catch所有情况的值，可以用_代替上述的变量other"),
    }

    //if let语法允许您将if和let组合成一种不那么详细的方式来处理匹配一个模式的值，而忽略其余的值
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _=>(),
    }
    //等价于
    if let Some(max) = config_max {
        println!("一样：{}",max);
    }
    //if let else语法
    if let Some(max) = config_max {
        println!("if,{}",max);
    } else {
        println!("else:");
    }

}

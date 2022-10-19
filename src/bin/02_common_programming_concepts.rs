fn main() {
    /***************1.变量可变性*******************/
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;  //Rust 的变量在默认情况下是不可变的，可以通过mut关键字让变量变为可变的
    println!("The value of x is: {x}");

    /***************2.使用下划线开头忽略未使用的变量*******************/
    let _not_used_variable_x = 5; //希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头
    // let not_used_variable_y = 10;

    /***************3.变量遮蔽*******************/
    //Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    let shadow = 5;

    let shadow = shadow + 1;

    {
        let shadow = shadow * 2;
        println!("The value of x in the inner scope is: {shadow}");
    }

    println!("The value of x is: {shadow}");

    //用处，变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量，就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
    let spaces = "   ";
    let spaces = spaces.len();

    /***************4.数据类型*******************/
    //（1）整形，有符号 i8,i16,i32,i64,i128,isize 无符号 u8,u16,u32,u64,u128,usize
    // isize和usize类型取决于运行程序的计算机的体系结构,如果您使用的是64位架构,则为64位,如果您使用的是32位,则为32位
    let _number_a = 10_000; //可以_用作视觉分隔符，使数字更易于阅读
    let _number_b = 100u32; //可以加后缀
    let _number_c = 0xff; //十六进制
    let _number_d = 0o77; //八进制
    let _number_e = 0b1111_0000; //二进制
    let _number_f = b'A'; //Byte (u8 only)
    //(2) 浮点数 f32, f64,和其它语言一样
    //数值操作
    // 加
    let _sum = 5 + 10;
    // 减
    let _difference = 95.5 - 4.3;
    // 乘
    let _product = 4 * 30;
    // 除
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // 结果为 0
    // 取余
    let _remainder = 43 % 5;
    //（3）布尔
    let _f = true;
    //（4）字符 
    //Rust的char类型大小为4个字节，表示Unicode标量值，这意味着它可以表示比ASCII多得多的内容。
    //重音字母;中文、日文、韩文;emoji;和零宽度空间都是Rust中的有效字符值。
    //Unicode标量值范围从U+0000到U+D7FF和U+E000到U+10FFFF包括在内。
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
    //（5）元组
    //元组是一种将许多具有各种类型的值分组为一种复合类型的通用方法。元组有固定的长度:一旦声明，它们的大小就不能增加或缩小。
    let tup:(i32, f64, u8)= (500, 6.4, 1);
    let (x, y, z) = tup; //可以通过模式匹配取出值
    println!("x={x},y={y},z={z}"); 
    //还可以这样取 
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    //(6)数组
    //另一种拥有多个值集合的方法是使用数组。与元组不同，数组的每个元素必须具有相同的类型。
    //与其他一些语言中的数组不同，Rust中的数组有固定的长度。
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let array_num = [3;5]; //let a = [3, 3, 3, 3, 3];
    //访问数组
    let _first = array_num[0];
    let _second = array_num[1];

    /***************5.函数*******************/
    let x = plus_one(5);
    println!("5+1={x}");
    //用大括号创建的新范围块是一个表达式
    //在这种情况下，计算结果为4. 该值被绑定为语句y 的一部分
    let y = {
        let x = 3;
        x + 1   //如果加分号则为语句，不会返回
    };
    println!("The value of y is: {y}");

    /***************6.控制流*******************/
    //if
    if_test1();
    if_test2();
    //loop
    loop_test();


}

fn plus_one(x:i32) -> i32 {
    x + 1
}


fn if_test1() {
    let num = 6;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//if在let语句中使用
//因为if是一个表达式，我们可以在语句的右侧使用它let 来将结果分配给一个变量
fn if_test2() {
    let condition = true;
    let number = if condition {
        5
    } else {
        4
    };
    println!("number is {number}");

    //块中的表达式if计算为整数，else块中的表达式计算为字符串。
    //这是行不通的，因为变量必须具有单一类型，并且 Rust 需要在编译时明确地知道 number变量是什么类型。
    //类型number可以让编译器在我们使用的任何地方验证该类型是否有效number。
    //number如果仅在运行时确定类型，Rust 将无法做到这一点；
    //如果编译器必须跟踪任何变量的多个假设类型，编译器会更复杂，并且对代码的保证更少。
    //let number = if condition { 5 } else { "six" };
    //println!("The value of number is: {number}");
}

fn loop_test() {
    //loop关键字告诉 Rust 永远一遍又一遍地执行一段代码，或者直到你明确告诉它停止。
    //可以使用break关键字跳出
    //用途1.
    //a 的用途之一loop是重试您知道可能会失败的操作，例如检查线程是否已完成其工作。
    //您可能还需要将该操作的结果从循环中传递到代码的其余部分。
    //为此，您可以在break用于停止循环的表达式之后添加您想要返回的值；该值将从循环中返回，以便您可以使用它
    println!("*********loop_test*************");
    let mut counter = 0;
    let num = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter * 2 is {}", num); //20

    //带标签的break
    println!("*********带标签的break*************");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while
    println!("*********while*************");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //for
    println!("*********for*************");
    let array_a = [1,2,3,4,5];
    for item in array_a {
        println!("value is:{item}");
    }

    for element in (1..4).rev() {
        println!("倒计时{element}!");
    }

}
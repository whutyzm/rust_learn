#[derive(Debug)]
struct User {
    active:bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Rust 还支持看起来类似于元组的结构，称为元组结构。
//元组结构具有结构名称提供的附加含义，但没有与其字段关联的名称；相反，它们只有字段的类型。
struct Point(i32, i32, i32);

//unit-like structs
struct AlwaysEqual;
fn main() {
    let mut user1= User {
        email: String::from("someone@qq.com"),
        username: String::from("yangzm"),
        active: true,
        sign_in_count: 1,
    };

    //如果实例是可变的，我们可以通过使用点表示法并分配给特定字段来更改值
    //注意整个实例必须是可变的；Rust 不允许我们只将某些字段标记为可变的。
    user1.email = String::from("2725262551@qq.com");
    println!("email:{}",user1.email);

    let user2 = build_user(String::from("yangzm"), String::from("someone@qq.com"));
    let user3 = build_user_field_init(String::from("yangzm"), String::from("someone@qq.com"));

    println!("{:?}", user2);
    println!("{:?}", user3);

    //在不使用更新语法的情况下定期创建新User实例
    let user4 = User {
        username: String::from("zhangsan"),
        email: user1.email,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("user4:{:?}",user4);
    //使用 struct update 语法，我们可以用更少的代码达到同样的效果
    let user5 = User {
        username: String::from("lisi"),
        ..user4
    };
    println!("user5:{:?}",user5);

    let point1 = Point(1,2,3);
    println!("point1:{},{},{}",point1.0, point1.1, point1.2);

 }

 fn build_user(username:String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
 }

 //参数名称和结构字段名称完全相同，我们可以使用field init 简写语法进行重写
 fn build_user_field_init(username:String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
 }

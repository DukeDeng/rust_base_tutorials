enum Direction{
    East,
    South,
    West,
    North,
}

#[warn(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[warn(dead_code)]
enum IpAddr {
    Ipv4,
    Ipv6
}

fn main (){
    about_match();
    let coin  = Coin::Dime;
    about_match_coin(coin);
    about_if_let();
    about_matches();
    about_cover();
    let x = Some(5);
    let y = about_option(x);
    let none = about_option(None);
    println!("y: {:?}", y);
    println!("none: {:?}", none);
}

/**
 * match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性。
match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同。
 */
fn about_match() {
    let dire = Direction::West;
    let _dire2 = Direction::East;
    let _dire3 = Direction::North;
    let _dire4 = Direction::South;
    // () 表示返回单元类型，而println!返回的也是单元类型
    match dire {
        Direction::West => println!("Going West"),
        Direction::East | Direction::North => println!("Going East or North"),
        _ => println!("Going somewhere else"),
    }
    let ip1 = IpAddr::Ipv4;
    // 使用 match 表达式可以赋值
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "xx.xx",
    };
    println!("ip_str: {}", ip_str);
}

fn about_match_coin(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Dime => 10,
        Coin::Nickel => 50,
        Coin::Quarter => 25,
    }
}

/**
 * 当你只要匹配一个条件，且忽略其他条件时就用 if let, 否者都用 match
 */
fn about_if_let() {
    let v = Some(3u8);
    if let Some(3) = v {
        println!("value is three");
    } 
}

/**
 * 将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
 */
fn about_matches(){
    let f = 'f';
    assert!(matches!(f, 'A'..='Z'|'a'..='z'));
}

/**
 * 变量覆盖：无论是 match 还是 if let， 他们都可以在模式匹配时覆盖掉老的值，绑定新的值。但是这种绑定只在当前作用域{}内有效
 */
fn about_cover() {
    let age = Some(3);
    println!("age: {:?}", age);
    if let Some(age) = age {
        println!("age: {}", age);
    }
    println!("age: {:?}", age);
}

/**
 * 一个变量要么有值：Some(T),要么为空：None.
 */
fn about_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
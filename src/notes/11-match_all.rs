struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Hello {id: i32},
}

fn main(){
    // 1. 匹配字面量 --- 希望匹配具体的可知的值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("tow"),
        _ => (),
    }

    //  2.匹配命名变量，存在变量覆盖问题
    let m = Some(5);
    let n = 10;
    match m {
        Some(50) => println!("Got 50"),
        Some(n) => println!("{}", n), // 这里的n是match作用域内声明的新变量n， 匹配到了5
        _ => println!("m is {:?}, n is {}", m, n), // 如果外面声明的m为None，进入了这里的模式，那么就是None 10
    }
    println!("m is {:?}, n is {}", m, n); // 输出结果：m is Some(5)

    // 3.可以使用 | 实现单分支多模式 类似于ts类型里的 |
    // Some(50) | Some(5) => println!("Got 50"),
    // 4. 可以通过 .. 以及 ..= 实现范围匹配
    // Some(1..=50) => println!("Got 50"),

    // 5. 关于模式匹配的解构，我的理解是 一定要长得一样。
    let p = Point { x: 10, y: 2 };
    let Point {x: a, y: b} = p; 
    println!("a is {}, b is {}", a, b);
    let Point { x: a, y: b } = Point { x: 1, y: 2 }; //长得一样 a就是1，b就是2
    println!("a is {}, b is {}", a, b);

    let Point{x, y} = p; // 这里无意间把上面的x变量覆盖掉了,简写形式: Point { x: x, y: y }= Point { x: 10, y: 2 }
    println!("x is {}, y is {}", x, y); // 10 原来的x是1 已经被覆盖

    // 下面解释了什么叫”长的一样“的匹配
    let msg = Message::ChangeColor(1, 2, 3);
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move {}, {}", x, y),
        Message::Write(text) => println!("Write {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to {}, {}, {}", r, g, b),
        _ => (),
    }

    // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件。
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 关于@绑定 @（读作 at）运算符允许为一个字段绑定另外一个变量。 需要注意@和｜一起使用时需要绑定到全模式
    let msg = Message::Hello {id:6};
    match msg {
        Message::Hello {id: id_val @ (6 | 1..=5)} => println!("id is {}", id_val),
        _ => (),    
    }
}
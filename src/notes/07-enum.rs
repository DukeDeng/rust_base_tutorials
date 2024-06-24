#[derive(Debug)]
enum PokerSuit {
    Clubs(char),
    Spades(char),
    Diamonds(char),
    Hearts(char),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main(){
    about_enum();
}

/**
 * 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举
 * 枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。
 */
fn about_enum() {
    let k = PokerSuit::Clubs('k');
    println!("{:?}", k);
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);
    println!("m1 is {:?}, m2 is {:?}, m3 is {:?}", m1, m2, m3);

    let msg = Message::Move{x:1,y:1};

    if let Message::Move{x:a,y:b} = msg {
        assert_eq!(a,b);
        return;
    } else {
        panic!("prevent this code run!");
    }
}
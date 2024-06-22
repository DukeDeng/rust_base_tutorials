fn main() {
    test_one();
    test_two();
}

fn test_one() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    // 变量X进入作用域，String类型的hello world被分配在堆上
    // let x = "hello, world"; // 错误，不能在栈上分配String类型    
    let y = x; // 变量y进入作用域，x的所有权被转移到y, x 失效
    // 2. let y = x.clone(); // 克隆x的所有权，x 仍然有效
    // 3. let y = x.as_str(); // 转化为&str类型，x 仍然有效
    // println!("{},{}", x, y); // error : 这里x无法被输出，y可以输出
    println!("{}", y); // 输出 "hello, world"
} // y 离开作用域被drop x 因为已经remove到了y，因此什么都不会操作

fn test_two() {
    let s = give_ownership();
    println!("{}", s); // 输出 "hello, world"
}

// 只能修改下面代码
fn give_ownership() -> String {
    let s = String::from("hello, world!");
    // convert String to Vec
    // let _s = s.into_bytes(); // error: into bytes 将 s所有权 转移到了_s.接下来s失效，应当使用as_bytes
    let _s = s.as_bytes(); // 转化为&[u8]类型，s 仍然有效
    s
}
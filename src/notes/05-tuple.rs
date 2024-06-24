fn main() {
    about_tuple();
}
/**
* 不同于数组要求每个元素必须是同一类型，元组中的成员可以是不同类型的。
*元组可以通过索引来访问每个成员，索引的起始是0。
过长的元组无法被打印输出（超过12个成员）。
元组可以作为函数的参数和返回值。
元组可以解构
*/
fn about_tuple(){
    // 这里元祖的最后一个成员是String类型，为了研究所有权问题
    let tup = (1, "hello", String::from("world"));
    let (x, y, _z) = tup;

    // println!("x is {}, y is {}, z is {}", x, y, tup.2); // error : tup中存在String 有所有权问题
    println!("x is {}, y is {}", x, y);
    // println!("tup is {:?}",tup) // error : tup中存在String 有所有权问题
    let (a, b);
    (a, b) = (true, 'x');
    println!("a is {}, b is {}", a, b);
}
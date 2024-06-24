fn main() {
    about_array();
}

/**
 * 在Rust中数组分为数组array和动态数组Vector。
 * 这两个数组的关系跟 &str 与 String 的关系很像，前者是长度固定的字符串切片，后者是可动态增长的字符串。
 * 这里说的数组是 Rust 的基本类型，是固定长度的，这点与其他编程语言不同，其它编程语言的数组往往是可变长度的，与 Rust 中的动态数组 Vector 类似。
 * 数组定义时必须是：1.长度已知 2.所有元素类型一致
 */
fn about_array() {
    let arr1 = [1, 2, 3];
    // 初始化一个某个值重复出现N次的数组， 1出现3次的数组
    let arr2 = [1; 3];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 访问数组中的某个元素的语法和js相同 arr[ind] 但是如果出现越界问题 在Rust中直接panic而不是返回一个不可预料的值，这点与js不同
    println!("arr1 is {:?}, arr2 is {:?}, a is {:?}", arr1, arr2, a);

    // let arr3 = [String::from("Hello"); 3]; // error : 数组在Rust中是基本类型,[value;length]的语法底层是Copy的实现，复杂类型都没有深拷贝，只能一个个创建
    // 不优雅的实现：
    let arr3 = [
        String::from("Hello"),
        String::from("Hello"),
        String::from("Hello"),
    ];
    println!("arr3 is {:?}", arr3);

    // 优雅的实现：
    // std::array::from_fn是一个函数，用于从一个返回值类型是 T 的闭包中创建一个大小为 N 的固定长度数组
    // 你需要提供一个闭包，该闭包将数组索引作为其参数，返回该索引处的值。
    // |i| 是 Rust 中闭包的参数语法。它表示一个匿名函数，该函数接受一个参数 i，并执行函数体中的代码
    let arr4: [String; 3] = std::array::from_fn(|i| String::from("hello"));
    println!("arr4 is {:#?}", arr4);
}
fn main(){
    about_slice();
    about_operate_str();
}
/**
 * 创建切片的语法，使用方括号包括的一个序列：[开始索引..终止索引]，前闭后开。

对于 let world = &s[6..11]; 来说，world 是一个切片，该切片的指针指向 s 的第 7 个字节(索引从 0 开始, 6 是第 7 个字节)，且该切片的长度是 5 个字节。
什么是字符串？在Rust中，字符串往往指的是String类型和&str字符串切片类型。
 */
fn about_slice(){
    let x = "helloworld";// 字符串字面量类型是&str 这也是字符串字面量不可变的原因，因为&str是不可变引用
    // 在通过索引区间来访问字符串时，需要格外的小心,需要注意索引是否落在边界的问题
    let y1 = &x[0..1];
    let y2 = &x[..1];
    let y3 = &x[5..];
    println!("y1 is {}", y1);
    println!("y2 is {}", y2);
    println!("y3 is {}", y3);
    let cn = "你好世界";
    // let z1 = &cn[0..2]; // panic 因为一个中文字符在utf8中占据3个字符，0..2没取到一个完整的
    let z2 = &cn[0..3];
    println!("z2 is {}", z2);
    let arr = [1, 2, 3, 4, 5];
    let arr1 = &arr[0..3];
    println!("arr1 is {:?}", arr1);
    let y = "hello";
    let z = y.to_string(); // &str 转 String
    let k: String = String::from("hello"); // String 字面量类型 
    let v = &z;
    let m = z.as_str(); // &str 转 &str
    let ref u = z; // String 转 &str
    println!("y is {},z is {},k is {},v is {},m is {},u is {}", y, z, k, v, m, u);
    //  Rust 不允许去索引字符串
    // println!("{}", z[0..]) // error : 在其它语言中，使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错
}

/**
*这里我的一个思考和理解：
因为追加/删除/插入/清空都是改变了原字符串的长度，因此这三个操作只能针对String类型，并且这个String类型还需要是mut的；
而replace/replacen 是替换原字符串的某段字符串并返回一个新的字符串，没有改变原字符串，所以可以操作String类型也能操作&str类型，并且如果是String类型也不需要用mut修饰；
至于replace_range方法虽然也是替换，但是它是直接操作原字符串不返回新的字符串，那么也就能理解它只能用于mut修饰的String类型了。
还有一点就是这里所有涉及到索引相关的操作都存在索引边界问题
*/
fn about_operate_str(){
    let mut x = String::from("hello");
    x.push(','); // 在末尾追加单个字符
    x.push_str("world"); // 在末尾追加字符串
    println!("x is {}", x);
    x.insert(0,'~');  // x.insert_str(idx, string) 在idx处插入字符串string
    println!("x is {}", x);

    let m: String = x.replace("~","##");
    println!("m is {}", m);
    let y = m.replacen("#", "@", 1); // 第三个参数是替换数量
    println!("y is {}", y);
    let mut z = String::from("中文");
    // z.replace_range(..2, "你"); // panic 2不在中的边界上
    z.replace_range(..3, "英"); 
    println!("z is {}", z);

    // 关于删除的方法
    // pop() 删除并返回最后一个字符 直接操作原来的字符 最后的返回值是一个Option类型，如果没有返回值就是None
    // remove(x)删除指定位置x上的字符，存在索引边界问题  直接操作原来的字符 返回值是删除位置的字符串
    // truncate(x) 删除字符串中从指定位置x开始到结尾的全部字符，无返回值，存在索引边界问题
    // clear() 删除所有字符 相当于truncate(0)
    // 关于连接的方法
    // 使用+或者+=连接字符串，要求右边的参数必须为字符串切片类型。因为调用+相当于调用std::string标准库的add()方法，这个方法的第二个参数要求的是一个引用类型。
    // +和+=都是i返回一个全新的字符串，因为被操作字符串不需要用mut修饰
    // format! 这种方式适用于 String 和 &str 对两边参数字符串类型没有要求
    let s1 = "hello";
    let s2 = String::from("world");
    let s = format!("{} {}!", s1, s2);
    println!("s is {}", s);
    // 字符串转义
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("byte_escape is {}", byte_escape);// I'm writing Rust
    // \u 可以输出一个 unicode 字符
    let unicode_escape = "\u{211D}";
    println!("unicode_escape is {}", unicode_escape);// ℝ
    // 如果希望字符串保持原样不要转义 用r""
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escape don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    // 如果字符串包含双引号，可以在开头和结尾加 # 必须是r#格式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    // 如果希望在字符串中使用 # 号，可以如下使用：
    let delimiter = r###"A string with "# in it. And even "##!"###;// 最少是r###
    println!("{}", delimiter);
    // 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法
    for c in "中国".chars() {
        println!("{}", c);
    }




}
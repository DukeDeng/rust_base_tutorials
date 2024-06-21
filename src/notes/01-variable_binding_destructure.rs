fn main() {
    about_unused_variable();
    about_bind_variablility_shadowing_scope();
    about_deconstruction();
}

/**
 * 未使用变量 & 格式化占位符
 * 注意函数名命名格式 a_b_c
 * 未使用变量通过_variable声明，避免warning
 * 关于 {} (显示格式化占位符) 和 {:?} (调试打印格式化占位符) 的区别：
 * {} 输出方式取决于类型实现的 std::fmt::Display trait
 * {:?}基于 Debug trait 实现的，用于将一个值格式化为可供调试输出的形式。对于 Rust 内建类型，一般会以比较友好的方式输出；
 * 可以通过 #[derive(Debug)] 来手动实现Debug trait，继而可以使用{:?}
 */
fn about_unused_variable() {
    let x: i32 = 2;
    let _y: i32 = 4;
    println!("x is {}", x);
    println!("x is {:?}", x);

    let s = String::from("hello");
    println!("Value of s is {}", s); // hello 可以看出， 使用 {} 占位符输出时，字符串 s 的引号被去掉了。它以更加自然的形势进行输出
    println!("Value of s is {:?}", s); // "hello"
    let  v = vec![1, 2, 3];
    println!("Value of v is {:?}", v); 

    // println!("Value of v is {}",v);// error[E0277]: `std::vec::Vec<i32>` doesn't implement `std::fmt::Display`

    #[derive(Debug)] // 手动实现Debug trait
    struct Person {
        name: String,
        age:i32,
    }
    let nick = Person {
        name: "nick".to_string(),
        age: 25,
    };
    // println!("nick is {}", Nick); // error: 因为Person没有实现`std::fmt::Display`

    println!("nick is {:?}", nick); //  如果没有 #[derive(Debug)] 则 error:因为Person没有实现`Debug`
                                    //  以下结构体.xx的输出直接可以使用,因为它们所代表的类型都默认实现了Display和Debug
    println!("nick name is {}", nick.name);
    println!("nick age is {}", nick.age);
    println!("nick name is {:?}", nick.name); 
    println!("nick age is {:?}", nick.age);
}

/**
 * 变量绑定 & 变量可见性 & 变量遮蔽 & 变量作用域
 */
fn about_bind_variablility_shadowing_scope() {
    let x: i32 = 1; // 将i32类型的值绑定到变量x上
    println!("x is {}", x);
    let mut y: i32 = 3;
    println!("y is {}", y);
    y = 4; // 重新绑定y的值
    y += 1;
    println!("y is {}", y);
    let mut z: i32 = 1; // 声明z，并初始化为1，未使用，被下面同名的z覆盖
    let mut z: i32 = 2; // 重新绑定z的值，但是z的作用域是局部的，不会影响到外部的z  
    println!("z is {}", z);
    let z = "xx";
    {
        let z = 100;
        println!("scope z is {}", z); // 局部变量z覆盖了外部的z，但是它只在当前作用域内有效
    }
    println!("z is {}", z); // 外部的z仍然是xx，因为它没有被覆盖
}

/**
 * 变量解构 & 解构赋值
 */
fn about_deconstruction() {
    let (mut x, y) = (1, 2);
    x = 4;
    println!("x is {}, y is {}", x, y);
    let (mut a, b):(i32,i32);
    let (mut a, _) = (1,2); // 2 没有意义
    let  (mut a, _) = (1,2); // error _ 只能用在左边
    (a,_) = (1,..);
    println!("a is {}", a); 
    [_, b] = [1, 3];
    println!("b is {}", b); // 解构赋值的右边可以是数组、元组、结构体等，也可以是变量、表达式、函数调用的结果
    let (x, y, z): (i32, i32, i32);
    (x, ..) = (1, ..);
    println!("x is {}", x);
}
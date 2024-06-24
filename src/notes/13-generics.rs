// 必须先<T>声明 去掉会报错
#[warn(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

enum Res<T, U> {
    Ok(T),
    Err(U),
}

fn main() {
    struct_enum_generics();
    methods_generics();

    let res = add(1, 2);
    println!("res is {}", res);
    let p1 = Point2 { x: 5, y: 10 };
    let p2 = Point2 { x: "hello", y: '中' };
    let p3 = p1.mixup(p2);
    // println!("p1 is {:?}", p1);// error p1的所有权被转移到p3了
    // println!("p2 is {:?}", p2);// error p2的所有权被转移到p3了
    println!("p3 is {:?}", p3);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}

/*
1. 所有范型的使用都必须提前声明
2. 一旦T的类型被确定，就无法被修改
 */
fn struct_enum_generics() {
    let p = Point { x: 1, y: 2 };
    // let n = Point { x: 1, y: 2.71 }; // error x是i32 这时候T已经确定为i32 y是f64 与T不匹配了

    let x = Point2 { x: 1, y: 2.71 }; // T U 可以不同
    println!("p is {:?}, x is {:?}", p, x);
}

// 方法中使用范型也需要提前声明 impl<T> 注意后面的Point<T>里的<T>不是声明范型，
// 而是一个完整的结构体类型，因为我们前面定义的结构体就是 Point<T> 而不再是 Point。
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn methods_generics() {
    let x = Point { x: 1, y: 2 };
    let txt =  x.x(); // 调用方法
    println!("txt is {}", txt);
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

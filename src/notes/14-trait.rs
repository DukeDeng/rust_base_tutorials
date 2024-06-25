use std::{
    fmt::{format, Debug, Display},
    iter::Sum,
};

/**
 * 特征 Trait：定义特征是 把一些方法组合在一起，目的是定义一个实现某些目标所必需的行为的“集合”
 * 特征只定义行为看起来是什么样的，而不定义行为具体是怎么样的。 类似 solidity 的接口。
 * 定义某个特征的语法示例：
 * trait Summary {
 *     fn summarize(&self) -> String;    // 方法签名
 * }
 * 下面，每一个实现这个特征的类型都需要实现该特征的相应方法。当然，可以定义默认实现的方法，其他类型可以选择实现或者重载
 * 孤儿规则：如果你想要的类型 A实现特征 T，那么 A 或者 T 至少有一个当前的作用域中定义的。可以确保其他人编写的代码的时候不会破坏你的代码。也确保了自己不会莫名其妙的就破坏了“不相干”的代码
 * 用处1：为不同的结构体实现相同的行为特征。 比如，可以为任何实现了 Display 特征的类型实现 Summary 特征，这样就可以在任何需要的时候调用 summarize 方法。 PS：方法也能做到
 * 用处2：特征可以作为函数的参数和返回值。
 */
struct Post {
    title: String,
    author: String,
    content: String,
}

struct Weibo {
    user: String,
    content: String,
}

struct Unknown {
    x: u8,
}

trait Summary {
    fn summarize(&self) -> String;
    // 这是特征的默认实现，具有这个特征的类型默认可以使用这个函数而不需要在impl for中实现，
    // 当然也可以对这个方法进行重载。注意默认实现即便没用到self也需要传入这个参数。

    fn defaultImpl(&self) {
        println!("默认实现");
    }
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章标题 {} by {}", self.title, self.author)
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("微博用户 {}，内容 {}", self.user, self.content)
    }
}

fn main() {
    let post = Post {
        title: "Rust 学习笔记".to_string(),
        author: "apple".to_string(),
        content: "Rust 学习笔记，主要介绍 Rust 基础语法和一些常用库的使用。".to_string(),
    };
    
    let weibo = Weibo {
        user: "apple".to_string(),
        content: "Rust 学习笔记。".to_string(),
    };

    // 用处1
    let sum1 = post.summarize();
    let sum2 = weibo.summarize();

    println!("sum1 is {}, sum2 is {}", sum1, sum2);

    let _unknown = Unknown { x: 10 };
    post.defaultImpl(); // 默认实现
    notify(&weibo); // 特征作为函数参数 微博用户测试用户,内容测试内容weibo
    // notify(&_unknown) // error : 因为Unknown结构体没有实现Summary特征
    notify6();
    let _struct = returns_summarizable(); // 返回值测试
    println!("_struct is {}", _struct.summarize()); // 调用summarize方法
}

/**
 * 用户2： 特征可以作为函数的参数和返回值。
 * 涉及到特征的约束概念：
 * 特征约束，可以让我们在“指定类型” + “指定特征”的条件下去实现方法。 如果没有特征约束，只能指定类型的实现方法。
 * item: impl Summary 可以理解为 实现了 (impl) Summary 特征的 item 作为参数。 可以使用任何实现了 Summary 特征的类型作为函数参数，
 * 同时在函数体内，还可以调用该特征的方法。这是语法糖，完整的写法：
 * fn notify<T: Summary>(item: &T) --- EX: T: Summary 被称为特征约束，表示 item 必须实现了 Summary 特征。
 * impl 特征xx这种语法糖在简单的场景下使用是没问题的，但是考虑一个场景，多个参数都需要实现了某一个特征的同一类型(因为实现某一特征的类型可以是不同的)，
 * 那么此时语法糖就无法起到有效的约束效果，这时候只能使用特征约束来实现
 * eg: pub fn notify(item1: &impl Summary, item2: &impl Summary) {} --- 这种写法 item1 和 item2 都可以实现不同的类型。
 * pub fn notify<T: Summary>(item1: &T, item2: &T) {} --- 这种写法 item1 和 item2 都必须实现了 Summary 特征。
 */
fn notify(item: &impl Summary){
    println!("通知：{}", item.summarize());
}

fn notify2<T: Summary>(item1: &T, item2: &T) {} // 特征约束
fn notify3<T: Summary + Display>(item: &T) {} // 多重约束 完整写法
fn notify4(item: &(impl Summary + Display)) {} // 多重约束 语法糖

// 在考虑一个问题，多重约束比较多的时候，这种完整写法显得冗余，这时候可以使用where约束
fn notify5<T, U, W, V>(t: &T, u: &U, w: &W, v: &V)
where 
    T: Display,
    U: Summary + Copy,
    W: Clone,
    V: Debug,
{}

struct Pair<T>{
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有T实现了Summary特征的Pair类型可以调用test方法
impl<T: Summary> Pair<T>{
    fn test(&self){
        println!("T 实现了Summary的Pair");
    }
}

fn notify6(){
    let pair1 = Pair{x: 10, y: 20};
    let pair2 = Pair {
        x: Weibo {
            user: "apple".to_string(),
            content: "Rust 学习笔记。".to_string(),
        },
        y: Weibo {
            user: "mike".to_string(),
            content: "Rust aadd".to_string(),
        },
    };
    // pair1.test(); // error pair1 中的 x 和 y 类型T 没有实现 Summary 特征
    pair2.test(); // 调用test方法
}

/**
 * 特征作为函数的返回值，这种形式只代表这个函数返回值实现了Summary特征，并不能具体的约束返回的类型，因为实现 Summary 特征的类型可以是不同的。比如在这里有weibo和Post两种类型
 * 使用场景：当我们不知道返回值的具体类型，也许是返回值的类型太复杂，只有编译器才能知道。
 * 例如：闭包和迭代器的类型。好在，可以用 impl Iterator 来告知调用者，返回了一个迭代器，因为所有迭代器都会实现 Iterator 特征。
 * 但是，这种返回值方式有一个很大的限制：只能有一个具体的类型。一旦第一个匹配到了一个类型，不能在更改。
 * 比如如果你想在返回值里使用 if else 来分别返回了实现这个特征的不同类型，是不被允许的
 */
fn returns_summarizable() -> impl Summary {
    Weibo{
        user: String::from("apple"),
        content: String::from("Rust 学习笔记。"),
    }
}
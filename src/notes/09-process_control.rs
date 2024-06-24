fn main() {
    about_process_control();
}

fn about_process_control() {
    let arr = ['a', 'b', 'c'];
    let arr2 = ["HELLO".to_string(), "WORLD".to_string()];
    let mut arr3:[String; 3] = std::array::from_fn(|i| String::from("name"));
    // 最普遍+最安全+效率最高
    // 注意，使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合，（比如我们这里使用了 container 的引用）
    // 如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了
    // 但是对于实现了 copy 特征的数组（例如 [i32; 10]）而言，不会存在这个问题
    for item in 0..5 {
        println!("{}", item)
    }

    // 需要访问索引
    // 具体来说， iter() 方法对数组进行迭代，而 enumerate() 方法在迭代过程中为每个元素提供索引。
    // 也可以用_替换v，通过arr[i]访问每个元素
    for (i, v) in arr.iter().enumerate() {
        println!("元素是 {}, 索引是 {}", v, i);
    }
    println!("{:?}", arr); // 没问题， 因为arr中的元素是实现了Copy特征的基本类型
    for item in arr2.iter() {
        println!("{}", item);
    }
    println!("{:?}", arr2);// error : 所有权转移了 因为arr2中的元素不是基础类型 
                            // 如果上面的循环是:in arr2.iter()就不会出现所有权问题了 因为当你使用 iter() 方法时，它会返回一个 Iterator，每个元素都是一个指向数组中字符串的引用（&String 类型）。
                            // 这种方式避免了复制字符串的开销，并且在迭代过程中也不会改变原始数组的数据。但是如果使用into_iter()则会转移所有权

    // iter_mut() 不会改变所有权 返回的是每个元素的可变引用
    for item in arr3.iter_mut() {
        item.pop();
        println!("item is {}", item);
    }
    println!("arr3 is {:?}", arr3);

    let mut n = 0;
    while n < 5 {
        println!("n is {}", n);
        n += 1;
    }

    // loop 循环 需要指定跳出循环的条件
    // continue 跳出本次循环
    // break 跳出整个循环
    let mut z = 0;
    loop {
        if z >= 3 {
            break;
        }
        println!("z is {}", z);
        z += 1;
    }
    // if 语句块是表达式，会返回值
    // 用 if 来赋值时，要保证每个分支返回的类型一样
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);
}

#[derive(Debug)]   
struct Person {
    name: String,
    age: u8,
    email: String,
    active: bool,
}
struct Point(i32, i32, i32);

fn main() {
    about_struct();
}

fn about_struct() {
    // 如果需要改变结构体内的数据，那么就把实例化的变量标记为mut，Rust不允许把结构体的某个字段标记为mut
    let mut gray = Person {
        name: "Gray".to_string(),
        age: 28,
        email: "gray@example.com".to_string(),
        active: true,
    };
    gray.age += 1;
    println!("{:#?}", gray);
    gray.email = "163".to_string();
    println!("{:#?}", gray);
    let mut mike = Person{
        name: String::from("Mike"),
        ..gray
    };
    mike.email = "mike@example.com".to_string();
    println!("{:#?}", mike);
    // println!("{:#?}", gray); // error gray的所有权在上面被转移到mike
    println!("{}", gray.age); // 29 age是copy 不存在所有权问题
    let Person{
        name,
        ref email,
        age,
        active,
    } = mike; // 这里的mike如果换成gray的话也会出现所有权问题。
              // 因为gray的email在前面已经move到了mike里，因此在这里使用的gray的email已经失效,
              // 需要注意的是gray的name在这里并未失效，因为在这之前name没有发生所有权的转移
    // println!("{}", gray.email); // 因为gray的email在前面已经move到了mike里，因此在这里使用的gray的email已经失效
    println!("{}", gray.name); 

    println!("{}", email);

    println!("{}", mike.email); // 因为上面使用的是ref email，因此email并没有被move，所以依旧可以使用，如果上面是email的话，这里就会因为所有权问题而报错

    // println!("{:#?}", mike); // error: mike 整体因为其中的字段发现了部分move，所以无法使用 
    
    let v = Point(1,2,3);
    // 解构
    // let (x, ..) = v; // error： 需要加上Point结构名才能解构
    let Point(x, ..) = v;
    let Point(x, _, z) = v;
    println!("{}, {}", x, z);
    println!("{}, {}, {}", x, v.1, z);
    println!("{}, {}, {}", x, v.1, v.2);
}
use std::collections::HashMap;
/**
 * HashMap key 的限制：任何实现了 Eq 和 Hash 特征的类型都可以作为 key
 * 包括 bool（基本不用，因为只有两种值）
 * int uint 以及他们的变体，例如 8u i32等
 * String 和 &str（提示：HashMap 的 key 是 String类型时，可以用 &str 作为 key， 配合get方法进行查询）
 * 需要注意的是, f32 和 f64 并没有实现Hash，因为浮点数精度问题导致无法进行相等比较。
 * 如果一个集合类型的所有字段都实现了 Eq 和 hash, 那该集合类型自动实现 Eq 和 Hash, 例如 Vec<T> 要实现 Hash, 那么首先需要 T 实现 Hash。
 * HashMap value 的限制：任何类型都可以作为 value
 */
fn main(){
    let mut team: HashMap<String, i32> = HashMap::new();
    team.insert("中国".to_string(), 10);
    team.insert(String::from("美国"), 25);
    println!("team is {:#?}", team);
    // 场景：将数据转化为hashmap类型 通过collect()，需要事先指定类型
    // into_iter 方法将列表转为迭代器，接着通过 collect 进行收集，不过需要注意的是，collect 方法在内部实际上支持生成多种类型的目标集合，
    // 因此我们需要通过类型标注 HashMap<_,_> 来告诉编译器类型
    let data = vec![
        ("中国".to_string(), 100),
        ("美国".to_string(), 10),
        ("德国".to_string(), 20),
    ];
    let mut newTeam: HashMap<String, i32> = data.into_iter().collect();
    println!("newTeam is {:#?}", newTeam);
    // 查询
    let score = newTeam.get(&"中国".to_string());
    println!("score is {:?}", score); // score is Some(100)

    let scoreValue = newTeam.get(&"中国".to_string()).copied().unwrap_or(0);
    println!("scoreValue is {:?}", scoreValue); // scoreValue is 100

    // 更新hashmap的值
    // 覆盖已有的值
    newTeam.insert("美国".to_string(), 12);
    println!("newTeam is {:#?}", newTeam);

    // 有过有就不插入，没有就插入
    newTeam.entry("中国".to_string()).or_insert(100);
    println!("newTeam is {:#?}", newTeam); // 不变，因为“中国”已存在

    let current = newTeam.entry("法国".to_string()).or_insert(200); // current 是 &mut的200
    *current += 20;
    println!("newTeam is {:#?}", newTeam); // "法国": 220
}
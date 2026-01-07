use serde::{Serialize, Deserialize};


//常规用法
#[derive(Serialize, Deserialize)]
// 将所有字段自动转成camelCase
#[serde(rename_all = "camelCase")]
struct Person {
    order_id: u64,
    // 单独改名
    #[serde(rename = "studentName")]
    name: String,

    // 忽略字段
    #[serde(skip)]
    student_age: u8,
    
    // 缺省值，使用Rust默认值
    #[serde(default)]
    address: String,

    // 缺省值，使用函数生成默认值
    #[serde(default = "default_age")]
    age: u8,
}

fn default_age() -> u8 {
    18
}


fn main() {
    let person = Person { 
        order_id: 1, 
        name: "John".to_string(), 
        student_age: 30,
        address: String::new(),  // 默认空字符串
        age: default_age(),      // 使用默认年龄函数
    };
    let serialized = serde_json::to_string(&person).unwrap();
    println!("{}", serialized);

    let json = "{\"orderId\":1,\"studentName\":\"John\"}";
    let person: Person = serde_json::from_str(json).unwrap();
    println!("age: {}", person.age);
    println!("address: {}", person.address);
}

// 1. 定义买卖方向
// 这里的 derive 就像 C++ 的宏，自动帮我们实现打印(Debug)、复制(Copy)、比较(PartialEq)等功能
#[derive(Debug, Clone, Copy)] 
enum Side {
    Bid, // 买
    Ask, // 卖
}

// 2. 定义订单结构体
#[derive(Debug, Clone, Copy)]
struct Order {
    id: u64,
    price: u64,
    qty: u64,
    side: Side,
}

// 3. 给结构体添加方法 (类似 C++ 类的方法实现)
impl Order {
    // Rust 没有 "new" 关键字，通常我们习惯写一个名为 new 的静态函数作为构造函数
    pub fn new(id: u64, price: u64, qty: u64, side: Side) -> Self {
        // Rust 如果变量名和字段名一样，可以简写 (比如 price: price 可以简写为 price)
        Self {
            id,
            price,
            qty,
            side,
        }
    }
}

fn main() {
    // 创建一个订单
    // let 是定义变量，mut 表示这个变量是"可变的" (mutable)
    // 如果不写 mut，Rust 默认变量是 const 的！
    let order1 = Order::new(1, 100, 50, Side::Bid);

    // 打印订单
    // {:?} 是调试打印占位符，它会自动调用 derive(Debug) 生成的代码
    println!("我的第一个订单: {:?}", order1);
    
    // 试试直接访问成员
    println!("订单价格是: {}", order1.price);
}
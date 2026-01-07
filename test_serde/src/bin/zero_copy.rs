use std::borrow::Cow;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MarketData<'a> {
    // 1. 对于肯定没有转义符的 ID、代码，直接用 &str 借用
    #[serde(borrow)]
    symbol: &'a str,      // 比如 "BTC/USD"，直接指过去，最快

    // 2. 对于可能有转义符的文本，用 Cow (写时复制)
    //    - 如果没转义符 -> 借用 (零拷贝)
    //    - 如果有转义符 -> 拷贝 (分配内存)
    #[serde(borrow)]
    description: Cow<'a, str>,

    // 3. 数字类型直接拷贝值，不需要引用
    price: f64,
    volume: u64,
}

fn handle_message(json: &str) -> MarketData {
    // 注意：解析出来的 msg，生命周期被绑死在 json 变量上
    let msg: MarketData = serde_json::from_str(json).unwrap();
    
    println!("{:?}", msg);
    msg
}

fn main() {
    let json = "{\"symbol\":\"BTC/USD\",\"description\":\"Bitcoin price\",\"price\":100000,\"volume\":100000}";
    let msg = handle_message(json);
    println!("{:?}", msg);
}
# Matching Engine

一个简单的撮合引擎实现，用于处理买卖订单的匹配。

## 功能特性

- 订单管理：支持买入（Bid）和卖出（Ask）订单
- 订单簿：使用 BTreeMap 维护价格排序的买卖订单簿
- 限价订单：支持按价格队列管理订单

## 项目结构

```
matching_engine/
├── src/
│   ├── order.rs    # 订单和买卖方向定义
│   ├── engine.rs   # 撮合引擎核心逻辑
│   └── main.rs     # 示例代码
└── Cargo.toml
```

## 核心组件

### Order（订单）

订单包含以下字段：
- `id`: 订单唯一标识
- `price`: 价格
- `qty`: 数量
- `side`: 买卖方向（Bid/Ask）
- `timestamp`: 时间戳

### MatchingEngine（撮合引擎）

撮合引擎维护两个订单簿：
- `bids`: 买单簿（BTreeMap<价格, 订单队列>）
- `asks`: 卖单簿（BTreeMap<价格, 订单队列>）

## 使用方法

```rust
use matching_engine::{MatchingEngine, Order, Side};

let mut engine = MatchingEngine::new();
let order = Order::new(1, 100, 50, Side::Bid, 0);
engine.process_order(order);
```

## 构建和运行

```bash
cd matching_engine
cargo build
cargo run
```


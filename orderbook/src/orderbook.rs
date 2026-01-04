use slab::Slab;
use std::collections::BTreeMap;
use rustc_hash::FxHashMap; // HFT 必备：极速 Hash

// 价格通常用整型表示 (比如 价格 * 10000)，避免浮点数 NaN 问题
type Price = u64; 
type OrderId = u64;
type OrderIndex = usize; // 这就是我们的“指针”

// ==========================================
// 1. 核心数据结构：订单节点
// ==========================================
// 注意：这里没有 Box，没有 Rc，只有纯数据。
// next 和 prev 是 Slab 中的索引，模拟双向链表。
#[derive(Debug)]
struct OrderNode {
    id: OrderId,
    price: Price,
    qty: f64,
    // 链表指针：使用 Option<usize>，None 表示空指针
    next: Option<OrderIndex>,
    prev: Option<OrderIndex>,
}

// ==========================================
// 2. 价格层级 (Price Level)
// ==========================================
// 这是一个队列的元数据，只存头和尾的索引
#[derive(Debug)]
struct PriceLevel {
    head: Option<OrderIndex>,
    tail: Option<OrderIndex>,
}

// ==========================================
// 3. 订单簿 (The Engine)
// ==========================================
pub struct OrderBook {
    // 内存池：所有订单都在这里，内存连续，缓存友好
    arena: Slab<OrderNode>,

    // 价格树：BTreeMap 在 Rust 中性能极高，且是有序的
    // Key: 价格, Value: 该价格下的队列指针
    bids: BTreeMap<Price, PriceLevel>,
    asks: BTreeMap<Price, PriceLevel>,

    // 快速查找表：通过 OrderId 瞬间找到 Slab 里的索引
    // 使用 FxHashMap 保证 O(1) 查找极快
    order_map: FxHashMap<OrderId, OrderIndex>,
}

impl OrderBook {
    // 初始化：预分配内存，避免运行时的扩容 (Reallocation)
    pub fn new(capacity: usize) -> Self {
        Self {
            arena: Slab::with_capacity(capacity),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            // 预分配 HashMap，避免 resize
            order_map: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    // ---------------------------------------------------
    // 动作 1: 下单 (Add Order) - 关键热路径
    // ---------------------------------------------------
    pub fn add_order(&mut self, id: OrderId, price: Price, qty: f64, is_bid: bool) {
        // 1. 在 Arena 中分配一个位置 (O(1)，只是数组操作)
        let entry = self.arena.vacant_entry();
        let idx = entry.key(); // 拿到这个位置的索引 (相当于指针地址)

        // 2. 写入数据
        entry.insert(OrderNode {
            id,
            price,
            qty,
            next: None,
            prev: None,
        });

        // 3. 记录 ID -> Index 映射
        self.order_map.insert(id, idx);

        // 4. 将索引挂到对应的价格层级链表上
        let book_side = if is_bid { &mut self.bids } else { &mut self.asks };
        
        // 获取或创建 PriceLevel
        let level = book_side.entry(price).or_insert(PriceLevel {
            head: None,
            tail: None,
        });

        // 标准的双向链表尾部插入操作
        match level.tail {
            Some(tail_idx) => {
                // 队列不为空：旧尾指向新节点，新节点前指旧尾
                self.arena[tail_idx].next = Some(idx);
                self.arena[idx].prev = Some(tail_idx);
                level.tail = Some(idx);
            }
            None => {
                // 队列为空：头尾都是新节点
                level.head = Some(idx);
                level.tail = Some(idx);
            }
        }
    }

    // ---------------------------------------------------
    // 动作 2: 撤单 (Cancel Order) - 关键热路径
    // ---------------------------------------------------
    pub fn cancel_order(&mut self, id: OrderId) -> bool {
        // 1. 查找索引 (O(1))
        let idx = match self.order_map.remove(&id) {
            Some(i) => i,
            None => return false, // 订单不存在
        };

        // 2. 获取订单信息 (为了知道去哪个价格层删)
        // 注意：先 copy 需要的信息，因为后面要 remove
        let (price, prev, next) = {
            let node = &self.arena[idx];
            (node.price, node.prev, node.next)
        };
        
        // 这里的逻辑稍微复杂，假设是 Bid (实际代码需要判断方向，这里简化演示)
        // 在真实系统中，你可以在 OrderNode 里存一个 is_bid 字段
        let level = self.bids.get_mut(&price).expect("Price level corruption");

        // 3. 链表断链操作 (Unlink)
        // 纯索引操作，无内存释放
        match (prev, next) {
            (Some(p), Some(n)) => {
                self.arena[p].next = Some(n);
                self.arena[n].prev = Some(p);
            }
            (Some(p), None) => {
                self.arena[p].next = None;
                level.tail = Some(p);
            }
            (None, Some(n)) => {
                self.arena[n].prev = None;
                level.head = Some(n);
            }
            (None, None) => {
                level.head = None;
                level.tail = None;
                // 可选：如果 Level 空了，从 BTreeMap 移除
                // self.bids.remove(&price); 
            }
        }

        // 4. 从 Arena 移除 (归还坑位，O(1))
        self.arena.remove(idx);
        true
    }
}

// ==========================================
// 简单测试
// ==========================================
fn main() {
    let mut book = OrderBook::new(100_000);

    // 预热：模拟进来 3 个买单
    book.add_order(1, 100, 1.0, true);
    book.add_order(2, 100, 2.0, true); // 同价格
    book.add_order(3, 101, 5.0, true); // 更高价格

    println!("Order 2 added. Index in slab: {:?}", book.order_map.get(&2));

    // 模拟撤单
    book.cancel_order(2);
    println!("Order 2 cancelled. Exists? {:?}", book.order_map.get(&2));
}

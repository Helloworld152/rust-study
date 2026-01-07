use std::mem;

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u64
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.id);
    }
}

fn main() {
    let button = Button { id: 0xDEADBEEF };
    let ptr_thin = &button;
    let ptr_fat: &dyn Draw = &button;

    println!("普通指针大小: {} bytes", mem::size_of_val(&ptr_thin));
    println!("胖指针大小:   {} bytes", mem::size_of_val(&ptr_fat));
    println!("---------------------------------");

    let (data_addr, vtable_addr): (usize, usize) = unsafe { 
        mem::transmute(ptr_fat) 
    };

    println!("Button 真实地址:     0x{:x}", ptr_thin as *const Button as usize);
    println!("胖指针里的 data 地址: 0x{:x}", data_addr);
    println!("胖指针里的 vtable地址: 0x{:x}", vtable_addr);

    // 验证：胖指针的第一部分必须等于对象的真实地址
    assert_eq!(ptr_thin as *const Button as usize, data_addr);
    println!("\n✅ 验证成功：胖指针的第一部分确实指向数据！");

    println!("---------------------------------");
    
    // --- 实验 3: 偷窥虚表 (Vtable) 内部 ---
    // 我们知道 vtable 里的第2个元素通常是对象的大小 (size)，第3个是对齐 (align)
    // 注意：这是编译器内部实现细节，未来可能会变，但目前通常是这样布局：
    // [destructor_ptr, size, align, method_ptr_1, ...]
    
    let vtable_ptr = vtable_addr as *const usize;
    unsafe {
        // 读取虚表里的第2个位置（索引1），应该是 Button 的大小
        let size_in_vtable = *vtable_ptr.add(1); 
        // 读取虚表里的第3个位置（索引2），应该是 Button 的对齐
        let align_in_vtable = *vtable_ptr.add(2);

        println!("Button 类型本身的大小: {} bytes", mem::size_of::<Button>());
        println!("虚表中记录的大小:     {} bytes", size_in_vtable);
        println!("虚表中记录的对齐:     {} bytes", align_in_vtable);
        
        if size_in_vtable == mem::size_of::<Button>() {
             println!("\n✅ 验证成功：虚表里确实记录了类型的大小信息！");
        }
    }
}
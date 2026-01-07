use std::mem::{size_of, transmute};

fn main() {
    let a: i32 = 10;
    let ptr: &i32 = &a;
    println!("普通指针大小 (&i32): {} 字节", size_of::<&i32>());
    
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3];
    println!("Slice指针大小 (&[i32]): {} 字节", size_of::<&[i32]>());

    let (ptr, len): (usize, usize) = unsafe {
        transmute(slice)
    };
    println!("Slice 指针地址: 0x{:x}", ptr);
    println!("Slice 长度: {} 字节", len);

    let s: &str = "Hello";
    println!("&str 指针大小: {} 字节", size_of::<&str>());
}
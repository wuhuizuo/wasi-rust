// 以下两种都可以的样子,当然深度的区分还需要了解什么什么时候需要使用 `extern "C"`

#[no_mangle]
pub fn greet1(a: i32) {
    println!("Hello, world {}!", a);
}

#[no_mangle]
pub extern "C" fn greet2(b: i32) -> i32 {
    println!("Hello, world {}!", b);
    b + 1
}

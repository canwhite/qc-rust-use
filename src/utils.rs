// utils.rs
// 访问修饰符，类似于java的使用
pub fn print_hello() {
    // 在Rust中，`!`表示这是一个宏调用而不是普通函数调用
    // `println!`是Rust的标准输出宏，它可以处理格式化字符串
    // 使用`!`的宏在编译时会展开成更复杂的代码，提供比普通函数更强大的功能
    // 在这里使用`!`是为了能够方便地格式化输出字符串
    println!("Hello from utils!");
}
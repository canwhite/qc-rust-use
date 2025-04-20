pub mod types{
    /**
    1. code reusability
    2. abstraction
    3. often used in structs , enums and function signatures
    */
    use std::fmt::Display;

    //function signatures
    pub fn genetics_function()-> (){
        // Add PartialOrd trait bound to enable comparison
        // PartialOrd 是 Rust 中的一个 trait，用于表示类型可以进行部分比较
        // 它继承了 PartialEq trait，并添加了比较操作符（<, >, <=, >=）的支持
        // 与 Ord trait 不同，PartialOrd 允许存在无法比较的情况（如浮点数中的 NaN）
        // 这里使用 PartialOrd 作为泛型约束，确保 T 类型可以进行大小比较
        fn largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];

            for item in list {
                // Use comparison with PartialOrd trait
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let a = [4,7,10,2];
        let b = largest(&a);
        //值引用，基本数据类型不需要:?, 当然注意不能直接输出
        println!("Largest number: {}", b);
    }

    //lifetime实际上就是变量的生命周期，还是在框架内
    // Rust 的类型推导主要在变量声明时起作用，对于函数返回值，如果不显式指定类型，通常会根据函数体内的返回值自动推导
    // 但是对于复杂情况，或者需要明确返回类型时，建议显式指定函数返回值类型
    pub fn use_lifetime() -> Option<String> {
        //if we want to return a referred data
        //The lifetime of a returned reference must come form the function's params
        //fn<x> ,<>里声明的是变量的lifetime
        
        // 演示生命周期的基本用法
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        let s1 = String::from("short");
        let s2 = String::from("longer string");
        let result = longest(s1.as_str(), s2.as_str());
        println!("Longest string: {}", result);

        // 返回一个 Option<String>
        Some("Lifetime example".to_string())
    }
}
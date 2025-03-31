pub mod types{
    /**
    1. code reusability
    2. abstraction
    3. often used in structs , enums and function signatures
    */
    use std::fmt::Display;

    //general type的概念和java是一致的  
    pub fn generics_struct()->(){
        #[derive(Debug)]  // Add Debug derive to enable printing
        struct Point<T>{
            x:T,
            y:T
        }

        let integer = Point{x:1,y:2};
        let float = Point{x:1.1,y:2.2};
        
        // Use debug formatting to print the structs
        println!("Integer Point: {:?}", integer);
        println!("Float Point: {:?}", float);
    }

    pub fn genetics_enum() -> Option<String>{
        //我们熟知的Option和Result就是这样来的
        enum Option<T> {
            Some(T),
            None
        }
        
        enum Result<T,E>{
            Ok(T),
            Err(E)
        }

        //更进一步
        enum TrafficLight{
            Red,
            Green,
            Yellow,
        }
        let yellow = TrafficLight::Yellow;

        //还有一种进阶版
        enum IpAddr{
            V4(String),
            V6(String),
        }
        //这种使用的时候伴随着初始化
        let home = IpAddr::V4(String::from("127.0.0.1"));

        // 返回一个 Option<String>
        Some("Enum example".to_string())
    }

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

    //trait，其实可以理解为接口的定义，然后在需要实现的时候实现，当然你也可以在定义的时候实现
    //实际上就是java接口的概念
    pub fn useOfTrait()->(){
        //定义接口，trait的作用和fn等同
        pub trait Summary {
            //里边定义方法，值引用，谁用引谁的self
            //当然也可以直接将summarize直接实现了
            fn summarize(&self)->String;
        }

        //接口实现或调用，satisfy a certain constraint /kənˈstreɪnt/
        struct Tweet {
            author:String,
            text: String,
        }

        // 这里的 for 关键字用于为特定类型实现 trait
        // 语法格式为：impl <trait名> for <类型名>
        // 这表示我们要为 Tweet 类型实现 Summary trait
    
        impl Summary for Tweet{
            fn summarize(&self)->String{
                //调用一下format，并返回格式化的字符串
                format!("{},{}", self.author, self.text)
            }
        }

        let tweet = Tweet{
            author:String::from("Zack"), 
            text:String::from("Hello world")
        };

        // 打印 tweet 的摘要
        let summary = tweet.summarize();
        println!("Tweet Summary: {}", summary);

        // -- 2）作为参数： callback，我这里直接给简化语法版--
        pub fn notify(item : &impl Summary){
            println!("{}",item.summarize());
        }

        // 可以传入一个实现了Summary trait的结构体实例
        notify(&tweet);

        //回字的第二种写法
        pub fn notify_1<T:Summary>(item1:&T,item2:&T){
            println!("{},{}",item1.summarize(),item2.summarize());
        } 

        //回字的第三种写法
        pub fn notify_2<T:Summary + Display>(item:&T){
            println!("{}",item.summarize());
        }
    }

    //lifetime实际上就是变量的生命周期，还是在框架内
    // Rust 的类型推导主要在变量声明时起作用，对于函数返回值，如果不显式指定类型，通常会根据函数体内的返回值自动推导
    // 但是对于复杂情况，或者需要明确返回类型时，建议显式指定函数返回值类型
    pub fn useOfVarLifetime() -> Option<String> {
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
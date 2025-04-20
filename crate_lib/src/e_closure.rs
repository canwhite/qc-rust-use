pub mod clo {

    pub fn closure(){

        // 1) 基本使用
        let num = 10;
        let read_num = || println!("Number: {}", num);
        read_num();


        // 2) 可变借用，这个根vec里的操作就差不多了
        let mut count = 0;
        let mut increment = || {
            //借用外边的值并变化
            count += 1;
            println!("Count: {}", count);
        };
        increment();

        
        // 3）获取所有权 ，move |x|{}
        // 因为内部输出了s，加上move所有权被借走，所以外部再输出就不ok
        let s = String::from("hello");
        let consume = move || {
            println!("Consumed: {}", s);
            // s 的所有权被移动到闭包内
        };
        consume();
        // println!("{}", s); // 错误！s 已被移动


        // 4) 闭包作为函数参数  where 关键字用于指定泛型约束 ， where F :后边是闭包的类型定义
        // Fn 是 Rust 中的一种 trait，表示闭包类型

        fn apply<F>(f: F) where F: Fn(i32) -> i32 {
            let result = f(5);
            println!("Result: {}", result);
        }
        apply(|x| x * 2); // 输出: Result: 10

        // 5) 闭包作为函数返回值
        // 作为返回值（需 Box 包装），dyn是在运行时确定具体类型
        fn create_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |b| a + b)
        }
        let adder = create_adder(3);
        println!("Sum: {}", adder(4)); // 输出: Sum: 7


        // 6）闭包和迭代器
        //macro define 
        let numbers = vec![1, 2, 3, 4];

        // iter() 方法用于创建一个迭代器，它允许我们按顺序访问集合中的每个元素， 在 Rust 中，iter() 是处理集合数据的常用方式，通常与 map、filter 等方法链式调用
        // collect() 的作用是将迭代器中的元素收集到一个集合中
        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        println!("Doubled: {:?}", doubled); // 输出: [2, 4, 6, 8]

        //这里还有一个筛选，注意前边的iter() 和后边的collect()
        // 在 Rust 中，`&` 用于创建引用，而 `*` 用于解引用。但是在这个特定的 filter 闭包中：
        // 1. `numbers.iter()` 返回的是 `&i32` 类型的迭代器
        // 2. 闭包参数 `x` 的类型是 `&&i32`（双重引用）
        // 3. 使用 `&&x` 实际上是模式匹配，它会自动解引用两次，直接得到 `i32` 类型的值
        // 4. 这种写法比 `*x` 更简洁，是 Rust 中的惯用写法
        // 5. 也可以写成 `|x| **x % 2 == 0` 或 `|&x| x % 2 == 0`，但 `&&x` 是最简洁的表达方式

        let even: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
        println!("Even: {:?}", even); // 输出: [2, 4]

    
    }
}
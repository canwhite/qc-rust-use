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


        // 4) 参数是closure，where相当于extends
        fn apply<F>(f: F) where F: Fn(i32) -> i32 {
            let result = f(5);
            println!("Result: {}", result);
        }
        apply(|x| x * 2); // 输出: Result: 10
        
        // 5) 返回值是closure , 需要Box包裹
        // dyn 是 Rust 中的动态分发关键字，用于表示 trait 对象
        fn create_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |b| a + b)
        }
        let adder = create_adder(3);
        println!("Sum: {}", adder(4)); // 输出: Sum: 7

        //WARN: 主要是作为参数和作为返回值这一块儿有区别
        //做参数的时候需要where定义它是什么？
        //做返回值的时候需要Box定义它是什么，多了个dyn

    }
}
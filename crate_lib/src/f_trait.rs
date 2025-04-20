pub mod test {

    use std::fmt::Display;

    pub fn use_trait()->(){
        
        pub trait Summary {
            fn summarize(&self)->String;
        }

        struct Tweet {
            author:String,
            text: String,
        }

        //for 给谁实现的接口
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

        // 方法后边加<>，无论无论是TS还是Java泛型的定义都是在名称之后
        pub fn _notify<T:Summary>(item1:&T,item2:&T){
            println!("{},{}",item1.summarize(),item2.summarize());
        } 


        // 上边这个notify如何调用
        let tweet1 = Tweet {
            author: String::from("Alice"),
            text: String::from("First tweet"),
        };

        let tweet2 = Tweet {
            author: String::from("Bob"),
            text: String::from("Second tweet"),
        };

        // 调用 _notify 函数
        _notify(&tweet1, &tweet2);
        
        
        //还可以和其他trait拼接
        pub fn _notify_1<T:Summary + Display>(item:&T){
            println!("{}",item.summarize());
        }
    }
}
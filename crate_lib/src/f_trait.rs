pub mod test {
    
    //trait，其实可以理解为接口的定义，然后在需要实现的时候实现，当然你也可以在定义的时候实现
    //实际上就是java接口的概念
    pub fn use_trait()->(){
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
        pub fn _notify_1<T:Summary>(item1:&T,item2:&T){
            println!("{},{}",item1.summarize(),item2.summarize());
        } 

        //回字的第三种写法
        pub fn _notify_2<T:Summary + Display>(item:&T){
            println!("{}",item.summarize());
        }
    }
}
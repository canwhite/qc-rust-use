//相当于将network里的分开的写法给合并了，pub mod，然后内部pub方法
//注意文件名必须是lib.rs

// Add necessary imports for the used types, 一层层的拿到
use std::collections::{VecDeque , LinkedList, HashMap, HashSet};

pub mod demos {
    
    // Import the external types，与super对应的还有内部的self调用
    use super::*;   

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    //返回空元组就等同于返回void 
    pub fn dataType()->(){
        //integer: 所有的变量值均为0，但是类型不同
        let a = 0_i8;
        //...
        let b = 0_i128;

        let c = 0_u8;
        //...
        let d = 0_u128;


        let e = 0_isize;
        let f = 0_usize;

        //float
        let a_1 = 0.0_f32;
        let b_1 = 0.0_f64;

        //bool
        let c_1 = true;
        let d_1 = false;

        //char 
        let e_1 = "3";


        //String
        let name_1 = String::from("Name");
        let name_2 = "name"; //相当于&str
        let name_3 = &name_1[0..3];
        // 使用 + 运算符拼接字符串
        let combined_1 = name_1 + " " + name_2;
        println!("使用 + 运算符拼接: {}", combined_1);

        // PS: 当然还可以format和new String去拼接，我们这里只了解最简单的一种

        //tuple
        let f_1 = (0_i128,"e",true);

        //array
        let g_1 = [1,2,3];     

        // 区间遍历
        for i in 0..g_1.len() {
            println!("数组第{}个元素是: {}", i, g_1[i]);
        }
        
        // 枚举遍历，前边是一个元组，也是用in
        for (index, value) in g_1.iter().enumerate() {
            println!("使用迭代器: 索引 {}, 值 {}", index, value);
        }

    }

    //container 仿佛都需要mut
    pub fn useOfContainer()->(){
        // 1) vector : 主要多了俩东西，一个是mut修饰符，一个vec!,vec!宏创建了一个可变的Vec
        // push pop insert和remove
        let mut vec = vec![1, 2, 3];

        // push
        vec.push(4);
        println!("添加元素后: {:?}", vec);
        
        // pop
        let last_element = vec.pop();
        println!("移除的元素: {:?}", last_element);
        println!("移除元素后: {:?}", vec);
  
        // insert
        vec.insert(1, 5);
        println!("在索引1插入5后: {:?}", vec);
        
        // remove
        vec.remove(2);
        println!("移除索引2的元素后: {:?}", vec);

        // slice 
        let r = &vec[0..1];


        // 2) vector queue，deque是双端队列的意思
        let mut v = VecDeque::new();
        v.push_back(0);
        v.push_front(0);
    

        // 3) LinkedList
        let mut l = LinkedList::new();
        l.push_back(0);

        // 4) HashMap   
        let mut m = HashMap::<i32,i32>::new();
        m.insert(1,1);
        let value = m.get(&1).expect("the key not exist");

        // 5) HashSet
        let mut s = HashSet::new();
        s.insert(0);

    }


    //complicated data type
    pub fn enumAndStruct()->(){

        //enum, 注意没有等号
        #[derive(Debug)]
        enum Month {
            Jan, //0
            Feb, //1
            Mar, //2
        }

        struct Person {
            //类型竟然是大写
            name:String,
            age:u8,
            address:String,
        }

        let month = Month::Jan;
        println!("当前月份是: {:?}", month);
        // println!("当前月份是: {}", month);

        let person = Person{
            name:String::from("Name"),
            age:16,
            address:String::from("Address"),
        };

        println!("Person的姓名是: {}", person.name);


    }

    
    pub fn FlowOfControl()->(){
        //for循环上边array已经有了，我们这里看下if
        let mut i = 0;
        //这python和c合到一块了
        if i > 0{
            println!("{i} is positive");
        }else{
            println!("{i} is negative")
        }

    }


    //option主要判断值的有无
    pub fn useOfOption()->Option<String>{
        // 在Rust中，函数的最后一行表达式如果不以分号结尾，则会被自动作为返回值
        None //返回值
    }

    //主要返回success和error
    pub fn useOfResult()->Result<(),u8>{
        Ok(())//不以分号结束，默认为返回值
    }

}
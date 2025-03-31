// Rust 中的 :: 是路径分隔符，用于访问模块中的项

// -- 引入文件 --
/***************************************************************
* package:rust中一个package对应一整个项目
* module:像我们utils就是一个最简单的module，mod 后边跟的是module文件名
* crate:是一个module的集合，use是使用包中定义内容，反而module名不重要了
***************************************************************/

// --mod跟的是文件名--
mod utils;
mod network;

// --use是使用具体的实现--
use rand::Rng;
use crate_lib::demos;
use crate_lib::types;
use crate_lib::concur;


fn main() {
    println!("Hello, world!");
    // 这里的 :: 不是类方法的意思，而是 Rust 中的路径分隔符
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("生成的随机数是: {}", secret_number);

    //根据路径调用,最后触及方法还是用::
    utils::print_hello();

    //同样根据路径调用
    network::client::connect();

    println!("3 + 5 = {}", demos::add(3, 5));

    //data type
    demos::dataType();

    //enum and struct
    demos::enumAndStruct();

    //flow
    demos::FlowOfControl();

    //安全选项option
    let a = demos::useOfOption();
    match a{
        Some(s) =>{
            println!("有值{}",s);
        },
        None=>{
            println!("无值");
        }
    }

    //注意可以使用if let来简化match过程
    //Some(3u8) 是一个 带类型标注的 Option 枚举值
    let some_u8_value = Some(3u8);

    //只是想匹配某个值，当这个值满足情况的下就去做什么事儿
    if let Some(3) = some_u8_value{
        println!("tree");
    }


    //result
    let b = demos::useOfResult();
    //注意match中和返回值的指向符不一样
    match b{
        //_可以匹配所有可能的取值
        Ok(_)=>{
            println!("成功返回");
        },
        Err(_)=>{
            println!("失败返回");
        }
    }


    //general types
    types::generics_struct();
    types::useOfTrait();


    //concurrent part

    concur::closure();
    concur::base_thread_of_usage();




}

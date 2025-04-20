// Rust 中的 :: 是路径分隔符，用于访问模块中的项

// -- 引入文件 --
/***************************************************************
* package:rust中一个package对应一整个项目
* module:像我们utils就是一个最简单的module，mod 后边跟的是module文件名
* crate:是一个module的集合，mod是添加子文件，use是使用包中定义内容，反而module名不重要了
***************************************************************/

// 如果只有一层拿着mode调就可以，如果有多层，需要有命名空间导出
mod utils; 
mod network; 
use crate_lib::base;
use crate_lib::base_types;
use crate_lib::container;


// use crate_lib::types;
// use crate_lib::concur;


fn main() {
    println!("Hello, world!");

    /* 
    这里的 :: 类似于命名空间定位的作用，是 Rust 中的路径分隔符
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("生成的随机数是: {}", secret_number);
    */

    //根据路径调用,最后触及方法还是用::
    utils::print_hello();

    //同样根据路径调用
    network::client::connect();

    println!("3 + 5 = {}", base::add(3, 5));

    //======data type=========
    base::data_type();



    //======option and result =======
    let a = base_types::use_option();
    match a{
        //=》
        Some(s) =>{
            println!("有值{}",s);
        },
        None=>{
            println!("无值");
        }
    }
    // 使用 if let 简化 Option 的匹配
    // if let就是确认一种类型,Some(s)是定义一个Some类型的值给赋值，然后在{}中判断
    // s是变量名
    if let Some(s) = base_types::use_option() {
        println!("有值{}", s);
    } else {
        println!("无值");
    }

    //result
    let b = base_types::use_result();

    match b{
        Ok(result)=>{
            println!("成功返回{:?}",result);
        },
        //_可以匹配所有可能的取值
        Err(_)=>{
            println!("失败返回");
        }
    }



    //=========container==========
    container::use_vec();
    container::use_hashmap();
    container::use_hashset();
    




    //general types
    // types::generics_struct();
    // types::useOfTrait();


    // //concurrent part

    // concur::closure();
    // concur::base_thread_of_usage();




}

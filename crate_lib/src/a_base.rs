
pub mod base {
    // Import the external types，与super对应的还有内部的self调用
    // use super::*;   

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    //鼓励的命名方法
    pub fn data_type()->(){
        
        let test_a = 1;
        let test_b = 2;
        let result = self::add(test_a, test_b);
        println!("result : {}",result);

        //integer: 所有的变量值均为0，但是类型不同
        let _a = 0_i8;
        //...
        let _b = 0_i128;

        let _c = 0_u8;
        //...
        let _d = 0_u128;


        let _e = 0_isize;
        let _f = 0_usize;

        //float
        let _a_1 = 0.0_f32;
        let _b_1 = 0.0_f64;

        //bool
        let _c_1 = true;
        let _d_1 = false;

        //char 
        let _e_1 = "3";

        //String
        let _name_1 = String::from("Name");
        let _name_2 = "name"; //相当于&str
        let _name_3 = &_name_1[0..3];
        // 使用 + 运算符拼接字符串
        let combined_1 = _name_1 + " " + _name_2;
        println!("使用 + 运算符拼接: {}", combined_1);

        //tuple
        let _f_1 = (0_i128,"e",true);

        //array
        let g_1 = [1,2,3];     


        let mut _i_1 = 0;

        //if 和for 语法像是 python和js的融合
        if _i_1 > 0{
            println!("{_i_1} is positive");
        }else{
            println!("{_i_1} is negative")
        }

        // for in , 0到g_1的length
        for i in 0..g_1.len() {
            println!("数组第{}个元素是: {}", i, g_1[i]);
        }
        
        // enumerate，注意链式语法
        for (index, value) in g_1.iter().enumerate() {
            println!("使用迭代器: 索引 {}, 值 {}", index, *value);
        }
    }

}
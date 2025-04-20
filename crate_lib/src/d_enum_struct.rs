pub  mod enum_struct {
    //complicated data type
    pub fn enum_struct()->(){

        //enum, 注意没有等号
        #[derive(Debug)]
        enum Month {
            _Jan, //0
            _Feb, //1
            _Mar, //2
        }

        struct Person {
            //类型竟然是大写
            _name:String,
            _age:u8,
            _address:String,
        }

        let month = Month::_Jan;
        println!("当前月份是: {:?}", month);
        // println!("当前月份是: {}", month);
        

        //WARN1：和c是一样的，不需要new
        let person = Person{
            _name:String::from("Name"),
            _age:16,
            _address:String::from("Address"),
        };
        //WARN2：和ENUM的区别是一个是寻址，一个是点语法
        println!("Person的姓名是: {}", person._name);
    }



    //general type的概念和java是一致的  
    pub fn generics_struct()->(){

        #[derive(Debug)]  // Add Debug derive to enable printing
        struct Point<T>{
            _x:T,
            _y:T
        }

        let integer = Point{_x:1,_y:2};
        let float = Point{_x:1.1,_y:2.2};
        
        // Use debug formatting to print the structs
        println!("Integer Point: {:?}", integer);
        println!("Float Point: {:?}", float);
    }
    


    pub fn genetics_enum() -> Option<String>{
        //我们熟知的Option和Result就是这样来的
        /* 
        enum Option<T> {
            Some(T),
            None
        }
        
        enum Result<T,E>{
            Ok(T),
            Err(E)
        }
        */

        //更进一步
        enum TrafficLight{
            _Red,
            _Green,
            _Yellow,
        }
        let _yellow = TrafficLight::_Yellow;

        #[allow(dead_code)]
        enum IpAddr{
            _V4(String),
            _V6(String),
        }
        //这种使用的时候伴随着初始化
        //WARN: String除了可以直接赋值，还可以from得到
        let _home = IpAddr::_V4(String::from("127.0.0.1"));

        // 返回一个 Option<String>
        // PS: String::from和to_string的区别？建议用String::from 
        // 1. String::from 是显式地从其他类型创建 String，更推荐使用
        // 2. to_string 是实现了 ToString trait 的类型的方法，会自动调用
        // 3. 性能上 String::from 通常更高效，因为它直接分配内存
        // 4. 代码风格上 String::from 更明确，to_string 更简洁
        // 5. 两者最终结果相同，都是创建新的 String 对象
        Some("Enum example".to_string())
    }

}
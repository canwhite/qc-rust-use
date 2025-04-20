use std::collections::{ HashMap, HashSet};

pub mod container {

    use super::*;

    //container 仿佛都需要mut
    pub fn use_vec()->(){

        /*=======================vector==============================
        主要多了俩东西，一个是mut修饰符，一个vec!,vec!宏创建了一个可变的Vec
        ===========================================================*/
        let mut vec = vec![1, 2, 3];

        // 1）push和pop
        vec.push(4);
        println!("添加元素后: {:?}", vec);
        let last_element = vec.pop();
        println!("移除的元素: {:?}", last_element);
        println!("移除元素后: {:?}", vec);

        // 2）insert
        vec.insert(1, 5);
        println!("在索引1插入5后: {:?}", vec);
        
        // 3）remove
        vec.remove(2);
        println!("移除索引2的元素后: {:?}", vec);

        // 4）alter 
        let _r = &vec[0..1]; //slice
    
        /* ========================================================
        WARN : 精髓
        elem默认是一个地址，*解引用，直接对值修改；
        和c++的不同之处，c++解引用是自动完成的，看起来像是直接对地址修改
        rust中可以使用地址传入的是绝大多数，除了零大小类型eg: ()，切片和Traits
        --总结：用这种写法吧，也好理解，参数默认是一个地址，大不了多少层的问题
        PS：如果你在参数里主动包了，就可以自动解包
        =========================================================*/
        if let Some(elem) = vec.get_mut(2){
            *elem = 5;
        }
        println!("get mut之后: {:?}", vec);

        //遍历修改
        //-- 抄python的多一点啊，index在前，item在后，注意这里用了可变循环，可以修改
        for (index,item) in vec.iter_mut().enumerate(){
            if index % 2 == 0 {
                *item *= 2; // 偶数索引元素翻倍_
            }
        }
        print!("遍历之后{:?}",vec);

        // 5)read
        //--position
        //闭包可以接大括号，也可以这样直接写
        let index =  vec.iter().position(|x|*x == 5);
        println!("等于5的位置:{:?}",index);
        
        if let Some(_s) = index{
            //Some可以看作一个包，这里解包
            println!("{:?}",index.unwrap());
        } 

        // --get, index和get配合
        let element = vec.get(index.unwrap());
        println!("get拿到的值:{:?}",element.unwrap());

        //-- contains
        // 使用contains方法检查vec中是否包含某个值
        let contains_five = vec.contains(&5);
        println!("vec是否包含5: {}", contains_five);

        //--filter
        //WARN: 这里注意iter一层，filter一层，所以是两层引用
        //注意加Vec<_>，因为在rust中，collect() 方法需要明确目标集合类型
        //其中_是一种占位符，允许rust推断
        let res:Vec<_> = vec.iter().filter(|x|**x >5).collect();
        println!("筛选后的结果:{:?}",res);


        //--into_iter转移所有权,可以跟sum和forEach
        let sum:i32 = vec.clone().into_iter().sum();
        println!("sum: {}",sum);
        //然后表演一波forEach,上边如果直接转移所有权，这意味着原集合将变为未初始状态，后续无法使用
        //所以我们尽量都加一个clone
        vec.clone().into_iter().for_each(|ele|{
            println!("elem: {}",ele);
        });

        //map操作
        let doubled: Vec<_> = vec![1, 2, 3].iter().map(|x| *x * 2).collect();
        println!("map doubled: {:?}",doubled);
        
        //最后还有一个类似reduce的操作，fold，哎呦这个有点屌
        //fold方法调用，先放一个init，然后和和单位值的lambda
        let fold_sum = vec![1, 2, 3].iter().fold(0, |acc, x| {
            acc +x
        });  // 6_
        println!("fold_sum:{:?}",fold_sum);


        //将vec转为map，关键在二维数组部分
        let new_vec = vec![("a", 1), ("b", 2)];
        //WARN: clone复制的是值，cloned复制的是元素
        let _map: HashMap<_, _> = new_vec.clone().into_iter().collect();
 

        vec.clear();
        // 因为 new_vec 是一个包含元组的 Vec，而元组中的元素是字符串和整数的组合
        // 字符串在 Rust 中是堆分配的，所以需要显式清理
        // 我们可以通过 drop 函数来确保资源被正确释放
        drop(new_vec);

        
        // _map.clear() 不成功的原因是因为我们在前面使用了 clone() 创建了 new_vec 的副本
        // 然后通过 into_iter() 将其转换为迭代器，最后 collect() 创建了新的 HashMap
        // 这个新的 HashMap 与原始的 new_vec 已经没有任何关系了
        // 所以当我们调用 _map.clear() 时，实际上是在清理一个已经和原始数据无关的 HashMap
        // 要正确清理数据，应该在创建 _map 之前就处理好所有权和清理问题
        // _map.clear();
        // BUT，我们能用drop清理
        drop(_map);

    }


    //HashMap
    pub fn use_hashmap(){
        // 4) HashMap   
        let mut m = HashMap::<i32,i32>::new();
        //insert
        m.insert(1,1);
        m.insert(2, 2);
        m.insert(3, 3);
        //get -- 通过key获取和删除
        let _value = m.get(&1).expect("the key not exist");
        //PS，get还有花活, 主要是这个copied and unwrap_or
        
        let _value_new = m.get(&1).copied().unwrap_or(0); 
        println!("_value_new:{:?}",_value_new);

        //remove
        m.remove(&1);
        println!("hashmap的删除之后{:?}",m);

        //total : 小结
        //和上边的vec一样都是，都有insert和remove
        //--keys, PS： 一般获取元素的时候,无论是keys还是values什么的，需要cloned～
        // cloned用于所有值，copied用于简单值，为了安全，都用cloned
        // cloned() 用于将迭代器中的引用转换为实际值，然后再collected转为数组，要不就是引用了
        let keys:Vec<_> = m.keys().cloned().collect();
        println!("keys: {:?}",keys);
        //--values，自己猜要怎么写

        //循环，简单的循环key value
        for (key,val) in &m{
            println!("key:value - {}: {}", *key, *val);
        }

        // 可变遍历HashMap,关键在于后边m的取值加了&mut
        for (key, val) in &mut m {
            *val += 1;  // 通过解引用修改值
            println!("修改后的 key:value - {}: {}", *key, *val);
        }

        // 另一种可变遍历方式
        m.iter_mut().for_each(|(key, val)| {
            *val *= 2;
            println!("再次修改后的 key:value - {}: {}", key, val);
        });

        //将map转为vec
        let vec:Vec<_> = m.iter().collect();
        println!("map to vec :{:?}",vec);


        //最后清空
        m.clear();

    }

    //HashSet
    pub fn use_hashset(){
        // 5) HashSet
        let mut s = HashSet::new();
        s.insert(0);
        s.insert(1);
        s.insert(2);
        s.insert(3);

        //remove-删除的是具体的值
        s.remove(&0);
        println!("after remove: {:?}",s);

        //get
        let _val =  s.get(&2).expect("error set value");
        println!("get value: {:?}",_val);

        //cotains
        let is_value = s.contains(&1);
        println!("is value: {:?}",is_value);

        // for in 
        // WARN: 在Rust中，vec和hashMap都有iter_mut()，但是set没有，所以不能&mut
        // 因为set在设计之初就是不允许改变的，除非remove之后在insert

        // 正确的HashSet遍历方式：
        for item in &s {
            println!("set item: {}", item);
        }

        //交并差的实现
        let s2: HashSet<i32> = HashSet::from([1, 2, 3, 4]);
        println!("初始化时赋值的 HashSet: {:?}", s2);

        // 或者使用 vec! 宏转换为 HashSet
        // let s2: HashSet<_> = vec![5, 6, 7, 8].into_iter().collect();
        // println!("使用 vec! 宏初始化的 HashSet: {:?}", s2);

        // intersection
        let common:HashSet<_> = s.intersection(&s2).collect();
        println!("common: {:?}",common);    
        
        // union
        let union:HashSet<_> = s.union(&s2).collect();  // 所有唯一元素[[2][24]]
        println!("union: {:?}",union);

        // difference
        let diff:HashSet<_> = s.difference(&s2).collect();  // 仅 set1 有[[2][24]]
        println!("diff : {:?}",diff);
        

    }

}
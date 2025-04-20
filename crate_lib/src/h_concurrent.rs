use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use reqwest;

pub mod concur{
    use super::*;


    pub fn base_thread_of_usage(){
        //spawn是引发的意思，/spɔːn/
        let handle = thread::spawn(||{
            println!("子线程执行");
        });  

        //join是等待结束，返回了Result需要unwrap
        handle.join().unwrap();

    }

    pub fn channel_of_usage(){
        //mpsc（多生产者单消费者）
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send("消息内容").unwrap();
        });
        println!("接收: {}", rx.recv().unwrap());
    }


    //使用mutex和Arc<T>原子引用技术来保证数据安全
    pub fn mutex_of_usage(){
        let counter = Arc::new(Mutex::new(0));

        // 用iter是为了拿到元素再闭包中使用，而这里只是多开几个线程
        let handles:Vec<_>  = (0..10).map(|_|{
            let c = Arc::clone(&counter);
            //然后启动线程，使用move获取所有权，然后+1
            thread::spawn(move || {
                let mut num = c.lock().unwrap();
                *num += 1;
            })
        }).collect(); 
        // 专程iter再forEach，这样forEach就能拿到单个item
        handles.into_iter().for_each(|h| h.join().unwrap());


    }

    async fn fetch_data() -> Result<String, reqwest::Error> {
        // ? 是 Rust 的错误处理运算符，用于简化 Result 类型的处理
        // 当遇到 Err 时，会立即返回错误，否则继续执行
        // 第一个 await 用于等待 HTTP 请求完成
        // 第二个 await 用于等待将响应体转换为字符串
        reqwest::get("https://example.com").await?.text().await
    }
    
    pub async fn async_and_await_usage() {
        //请求结果的整合
        match self::fetch_data().await {
            Ok(data) => println!("{}", data),
            Err(e) => eprintln!("请求失败: {:?}", e),
        }
    }

    pub fn atonic_of_usage(){
        let count = AtomicUsize::new(0);
        //支持多种内存顺序（如 Relaxed、SeqCst），平衡性能与一致性需求 
        count.fetch_add(1, Ordering::Relaxed);
    }

}
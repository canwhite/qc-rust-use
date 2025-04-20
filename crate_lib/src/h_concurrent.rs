// use std::thread;
// use std::sync::mpsc;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::{Arc, Mutex};
use tokio;

pub mod concur{
    use super::*;

    //thread pool - worker_threads
    pub fn tokio_spawn_example() -> Result<i32, Box<dyn std::error::Error>> {
        // 创建 Tokio 运行时

        let rt = tokio::runtime::Builder::new_current_thread()
            .worker_threads(8)
            .enable_io()
            .build()?;

        // block_on是个容器，但是可以返回值
        let result = rt.block_on(async {
            let handle = tokio::spawn(async {
                println!("Tokio spawned task is running!");
                42 // 返回一个示例值
            });

            // 等待任务结束并解包
            handle.await.unwrap()
        });

        Ok(result)
    }

    //mpsc
    pub async fn tokio_mpsc_example() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // 创建一个异步 MPSC 通道，缓冲区大小为 32
        // 返回值是一个transit，一个receive
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        // 生产者任务：发送消息到通道
        let producer = tokio::spawn(async move {
            for i in 0..5 {
                if let Err(e) = tx.send(format!("Message {}", i)).await {
                    eprintln!("Failed to send message: {}", e);
                }
            }
        });

        // 用于存储接收到的消息
        let mut received_messages = Vec::new();

        // 接收消息并存储到 Vec 中，等待接受完成
        while let Some(message) = rx.recv().await {
            received_messages.push(message);
        }

        // 等待生产者任务完成
        producer.await?;

        Ok(received_messages)
    }

    // 示例：使用 Tokio 的互斥锁保护共享数据
    pub async fn tokio_mutex_example() -> Result<i32, Box<dyn std::error::Error>> {
        // 创建一个被 Mutex 包裹的共享计数器，使用 Arc 包裹 Mutex，以便在多个任务之间共享
        let counter = std::sync::Arc::new(tokio::sync::Mutex::new(0));

        // 创建多个任务来修改计数器
        let handles: Vec<_> = (0..5).map(|_| {
            // 使用 Arc::clone 创建共享引用
            let counter_clone = std::sync::Arc::clone(&counter);
            // total: move在执行异步任务的时候极其有用，因为任务可能在其他线程执行，需要把所有权转移
            tokio::spawn(async move {
                let mut num = counter_clone.lock().await;
                *num += 1;
            })
        }).collect();

        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }

        // Mutex.lock() 返回一个 MutexGuard，它实现了 Deref trait， 通过 * 操作符可以访问被保护的数据
        let final_value = *counter.lock().await;
        Ok(final_value)
    }



    /*=========================原生操作================================

    //1）thread spawn
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

    //2) mutex
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
    
    //atonic
    pub fn atonic_of_usage(){
        let count = AtomicUsize::new(0);
        //支持多种内存顺序（如 Relaxed、SeqCst），平衡性能与一致性需求 
        count.fetch_add(1, Ordering::Relaxed);
    }
    ================================================================*/



}
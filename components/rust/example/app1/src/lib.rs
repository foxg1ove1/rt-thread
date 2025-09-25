#![no_std]

extern crate alloc;

use alloc::string::String;
use rtt_main::rtt_main;
use rtt_rs2::param::Param;
use rtt_rs2::{println, Thread, Semaphore, Mutex, MessageQueue, delay_ms};
use core::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

// 简单的演示程序
#[rtt_main(appname="demo", run=true, cmd=true, desc="Rust demo app")]
fn main(param: Param) {
    println!("Hello from Rust!");
    println!("Parameters count: {}", param.len());
    
    for (i, arg) in param.enumerate() {
        if let Ok(s) = String::from_utf8(arg) {
            println!("Arg {}: {}", i, s);
        }
    }
    
    // 演示线程创建
    demo_thread();
    
    // 演示信号量
    demo_semaphore();
    
    // 演示互斥量  
    demo_mutex();
    
    // 演示消息队列
    demo_message_queue();
}

fn demo_thread() {
    println!("=== Thread Demo ===");
    
    let thread = Thread::new("rust_thread", 2048, 10, || {
        for i in 0..5 {
            let count = COUNTER.fetch_add(1, Ordering::SeqCst);
            println!("Thread iteration {}, global counter: {}", i, count);
            delay_ms(1000);
        }
        println!("Thread finished");
    }).expect("Failed to create thread");
    
    thread.start().expect("Failed to start thread");
    
    // 等待线程完成
    delay_ms(6000);
}

fn demo_semaphore() {
    println!("=== Semaphore Demo ===");
    
    let sem = Semaphore::new("test_sem", 2).expect("Failed to create semaphore");
    
    println!("Taking semaphore...");
    sem.take(None).expect("Failed to take semaphore");
    println!("Semaphore taken");
    
    println!("Releasing semaphore...");
    sem.release().expect("Failed to release semaphore");
    println!("Semaphore released");
}

fn demo_mutex() {
    println!("=== Mutex Demo ===");
    
    let mutex = Mutex::new("test_mutex").expect("Failed to create mutex");
    
    {
        let _guard = mutex.lock().expect("Failed to lock mutex");
        println!("Mutex locked, doing work...");
        delay_ms(100);
        println!("Work done");
        // guard自动释放互斥量
    }
    
    println!("Mutex unlocked");
}

fn demo_message_queue() {
    println!("=== Message Queue Demo ===");
    
    let mq = MessageQueue::new("test_mq", 64, 10).expect("Failed to create message queue");
    
    let test_msg = b"Hello from message queue!";
    
    println!("Sending message...");
    mq.send_bytes(test_msg).expect("Failed to send message");
    
    println!("Receiving message...");
    match mq.try_recv_bytes() {
        Ok(received) => {
            if let Ok(s) = String::from_utf8(received) {
                println!("Received: {}", s.trim_end_matches('\0'));
            }
        }
        Err(e) => println!("Failed to receive message: {:?}", e),
    }
}

// 另一个简单的测试命令
#[rtt_main(appname="test", cmd=true, desc="Simple test command")]
fn test_command(_param: Param) {
    println!("Test command executed!");
    
    for i in 0..3 {
        println!("Count: {}", i);
        delay_ms(500);
    }
}
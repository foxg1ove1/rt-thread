#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate alloc;

// 子模块
pub mod bind;
pub mod malloc;
pub mod out;
pub mod puts;
pub mod param;
pub mod thread;
pub mod semaphore;
pub mod mutex;
pub mod queue;
pub mod time;
pub mod prelude;

// API 模块
pub mod api;

// 重新导出核心功能
pub use bind::*;
pub use malloc::*;
pub use out::*;
pub use puts::*;
pub use param::Param;
pub use thread::*;
pub use semaphore::*;
pub use mutex::*;
pub use queue::*;
pub use time::*;

// 重新导出常用类型
pub use alloc::{
    string::{String, ToString},
    vec::Vec,
    format,
    boxed::Box,
    collections::BTreeMap,
};

// 全局分配器
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// 初始化堆内存
pub fn init_heap() {
    extern "C" {
        static mut __heap_start: u8;
        static mut __heap_end: u8;
    }
    
    unsafe {
        let heap_start = &mut __heap_start as *mut u8;
        let heap_end = &mut __heap_end as *mut u8;
        let heap_size = heap_end as usize - heap_start as usize;
        
        if heap_size > 0 {
            ALLOCATOR.lock().init(heap_start, heap_size);
        }
    }
}

// Panic 处理
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        rt_kprintf(b"[PANIC] Rust panic occurred!\n\0".as_ptr() as *const i8);
        
        if let Some(location) = info.location() {
            let mut buffer = [0u8; 256];
            let file = location.file().as_bytes();
            let line = location.line();
            
            // 构造错误信息
            let mut pos = 0;
            let prefix = b"File: ";
            let len = prefix.len().min(buffer.len() - pos - 1);
            buffer[pos..pos + len].copy_from_slice(&prefix[..len]);
            pos += len;
            
            let file_len = file.len().min(buffer.len() - pos - 20);
            buffer[pos..pos + file_len].copy_from_slice(&file[..file_len]);
            pos += file_len;
            
            let line_prefix = b" Line: ";
            let len = line_prefix.len().min(buffer.len() - pos - 10);
            buffer[pos..pos + len].copy_from_slice(&line_prefix[..len]);
            pos += len;
            
            // 简单的数字转换
            let mut line_num = line;
            let mut line_digits = [0u8; 10];
            let mut line_pos = 0;
            
            if line_num == 0 {
                line_digits[0] = b'0';
                line_pos = 1;
            } else {
                while line_num > 0 {
                    line_digits[line_pos] = (line_num % 10) as u8 + b'0';
                    line_num /= 10;
                    line_pos += 1;
                }
                line_digits[..line_pos].reverse();
            }
            
            let line_len = line_pos.min(buffer.len() - pos - 1);
            buffer[pos..pos + line_len].copy_from_slice(&line_digits[..line_len]);
            pos += line_len;
            
            buffer[pos] = 0;
            
            rt_kprintf(buffer.as_ptr() as *const i8);
            rt_kprintf(b"\n\0".as_ptr() as *const i8);
        }
        
        if let Some(msg) = info.message() {
            rt_kprintf(b"Message: \0".as_ptr() as *const i8);
            // 这里只能打印简单信息，避免再次panic
            rt_kprintf(b"(message details omitted)\n\0".as_ptr() as *const i8);
        }
    }
    
    loop {
        unsafe {
            rt_thread_delay(1000);
        }
    }
}
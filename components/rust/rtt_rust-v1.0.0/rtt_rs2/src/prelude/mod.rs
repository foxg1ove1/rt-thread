// Prelude 模块 - 常用导入

pub mod no_std;

// 重新导出常用类型和函数
pub use crate::{
    // 基础绑定
    bind::{RtError, RtResult},
    
    // 线程
    thread::{Thread, current_thread, yield_now, delay_ms, delay_ticks, delay_s, delay_us},
    
    // 同步原语
    semaphore::{Semaphore, BinarySemaphore},
    mutex::{Mutex, MutexGuard},
    queue::MessageQueue,
    
    // 时间
    time::{Time, Timer},
    
    // 参数
    param::Param,
    
    // 打印
    print, println, debug_print, error_print, warn_print, info_print,
    
    // 常用容器
    Vec, String, Box,
    
    // 内存管理
    malloc::{rt_malloc_safe, rt_free_safe, get_memory_info, print_memory_info},
};

// 便利的Result类型
pub type Result<T> = RtResult<T>;
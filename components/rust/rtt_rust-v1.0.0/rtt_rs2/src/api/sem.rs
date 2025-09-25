// 高级信号量API

use crate::{Semaphore, RtResult};
use alloc::string::String;

/// 信号量构建器
pub struct SemaphoreBuilder {
    name: String,
    value: u32,
    is_prio: bool,
}

impl SemaphoreBuilder {
    /// 创建新的信号量构建器
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: 0,
            is_prio: false,
        }
    }
    
    /// 设置初始值
    pub fn value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }
    
    /// 设置为优先级信号量
    pub fn priority(mut self) -> Self {
        self.is_prio = true;
        self
    }
    
    /// 构建信号量
    pub fn build(self) -> RtResult<Semaphore> {
        if self.is_prio {
            Semaphore::new_prio(&self.name, self.value)
        } else {
            Semaphore::new(&self.name, self.value)
        }
    }
}

/// 便利函数：创建计数信号量
pub fn counting_semaphore(name: &str, count: u32) -> RtResult<Semaphore> {
    SemaphoreBuilder::new(name).value(count).build()
}

/// 便利函数：创建二进制信号量
pub fn binary_semaphore(name: &str) -> RtResult<Semaphore> {
    SemaphoreBuilder::new(name).value(1).build()
}

/// 便利函数：创建同步信号量（初始为0）
pub fn sync_semaphore(name: &str) -> RtResult<Semaphore> {
    SemaphoreBuilder::new(name).value(0).build()
}
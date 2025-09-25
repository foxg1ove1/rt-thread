// 高级线程API

use crate::{Thread, RtResult, RtError};
use alloc::string::String;

/// 线程构建器
pub struct ThreadBuilder {
    name: String,
    stack_size: u32,
    priority: u8,
    tick: u32,
}

impl ThreadBuilder {
    /// 创建新的线程构建器
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stack_size: 2048,
            priority: 10,
            tick: 20,
        }
    }
    
    /// 设置栈大小
    pub fn stack_size(mut self, size: u32) -> Self {
        self.stack_size = size;
        self
    }
    
    /// 设置优先级
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
    
    /// 设置时间片
    pub fn tick(mut self, tick: u32) -> Self {
        self.tick = tick;
        self
    }
    
    /// 生成线程
    pub fn spawn<F>(self, f: F) -> RtResult<Thread>
    where
        F: FnOnce() + Send + 'static,
    {
        Thread::new(&self.name, self.stack_size, self.priority, f)
    }
    
    /// 生成并启动线程
    pub fn spawn_and_start<F>(self, f: F) -> RtResult<Thread>
    where
        F: FnOnce() + Send + 'static,
    {
        let thread = self.spawn(f)?;
        thread.start()?;
        Ok(thread)
    }
}

/// 便利函数：快速创建并启动线程
pub fn spawn<F>(name: &str, f: F) -> RtResult<Thread>
where
    F: FnOnce() + Send + 'static,
{
    ThreadBuilder::new(name).spawn_and_start(f)
}

/// 便利函数：创建后台线程（低优先级）
pub fn spawn_background<F>(name: &str, f: F) -> RtResult<Thread>
where
    F: FnOnce() + Send + 'static,
{
    ThreadBuilder::new(name)
        .priority(20)
        .spawn_and_start(f)
}

/// 便利函数：创建高优先级线程
pub fn spawn_high_priority<F>(name: &str, f: F) -> RtResult<Thread>
where
    F: FnOnce() + Send + 'static,
{
    ThreadBuilder::new(name)
        .priority(5)
        .spawn_and_start(f)
}
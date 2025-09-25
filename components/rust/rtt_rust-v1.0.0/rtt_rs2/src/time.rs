// 时间相关功能

use crate::thread::{delay_ms, delay_ticks, delay_s, delay_us};

/// 时间工具结构
pub struct Time;

impl Time {
    /// 延时毫秒
    pub fn delay_ms(ms: u32) {
        delay_ms(ms)
    }
    
    /// 延时tick
    pub fn delay_ticks(ticks: u32) {
        delay_ticks(ticks)
    }
    
    /// 延时秒
    pub fn delay_s(s: u32) {
        delay_s(s)
    }
    
    /// 延时微秒（近似）
    pub fn delay_us(us: u32) {
        delay_us(us)
    }
}

/// 简单的计时器
pub struct Timer {
    start_tick: u32,
}

impl Timer {
    /// 创建新的计时器
    pub fn new() -> Self {
        Self {
            start_tick: Self::get_tick(),
        }
    }
    
    /// 重置计时器
    pub fn reset(&mut self) {
        self.start_tick = Self::get_tick();
    }
    
    /// 获取已经过的tick数
    pub fn elapsed_ticks(&self) -> u32 {
        Self::get_tick().wrapping_sub(self.start_tick)
    }
    
    /// 获取已经过的毫秒数（近似）
    pub fn elapsed_ms(&self) -> u32 {
        // RT-Thread 默认tick是1ms，但这里应该根据实际配置
        self.elapsed_ticks()
    }
    
    /// 获取当前系统tick
    fn get_tick() -> u32 {
        unsafe {
            extern "C" {
                fn rt_tick_get() -> u32;
            }
            rt_tick_get()
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// 测量代码执行时间的宏
#[macro_export]
macro_rules! time_it {
    ($name:expr, $block:block) => {
        {
            let timer = $crate::time::Timer::new();
            let result = $block;
            let elapsed = timer.elapsed_ms();
            $crate::println!("{} took {} ms", $name, elapsed);
            result
        }
    };
}
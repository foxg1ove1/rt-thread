// 基础API封装

use crate::bind::*;

/// 系统信息
pub struct SystemInfo {
    pub total_memory: u32,
    pub used_memory: u32,
    pub max_used_memory: u32,
}

impl SystemInfo {
    /// 获取系统信息
    pub fn get() -> Self {
        let (total, used, max_used) = crate::malloc::get_memory_info();
        Self {
            total_memory: total,
            used_memory: used,
            max_used_memory: max_used,
        }
    }
    
    /// 获取空闲内存
    pub fn free_memory(&self) -> u32 {
        self.total_memory.saturating_sub(self.used_memory)
    }
    
    /// 获取内存使用率（百分比）
    pub fn memory_usage_percent(&self) -> u32 {
        if self.total_memory == 0 {
            0
        } else {
            (self.used_memory * 100) / self.total_memory
        }
    }
}

/// 系统控制
pub struct System;

impl System {
    /// 重启系统
    pub fn reboot() -> ! {
        unsafe {
            extern "C" {
                fn rt_hw_cpu_reset() -> !;
            }
            rt_hw_cpu_reset();
        }
    }
    
    /// 关闭中断
    pub fn disable_interrupt() -> u32 {
        unsafe {
            extern "C" {
                fn rt_hw_interrupt_disable() -> u32;
            }
            rt_hw_interrupt_disable()
        }
    }
    
    /// 恢复中断
    pub fn enable_interrupt(level: u32) {
        unsafe {
            extern "C" {
                fn rt_hw_interrupt_enable(level: u32);
            }
            rt_hw_interrupt_enable(level);
        }
    }
    
    /// 进入临界区（返回中断级别）
    pub fn enter_critical() -> u32 {
        Self::disable_interrupt()
    }
    
    /// 退出临界区
    pub fn exit_critical(level: u32) {
        Self::enable_interrupt(level);
    }
}

/// 临界区守卫
pub struct CriticalSection {
    level: u32,
}

impl CriticalSection {
    /// 进入临界区
    pub fn new() -> Self {
        Self {
            level: System::enter_critical(),
        }
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        System::exit_critical(self.level);
    }
}

/// 临界区宏
#[macro_export]
macro_rules! critical_section {
    ($block:block) => {
        {
            let _guard = $crate::api::CriticalSection::new();
            $block
        }
    };
}
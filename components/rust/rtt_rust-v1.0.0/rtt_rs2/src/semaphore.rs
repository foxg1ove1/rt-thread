// 信号量封装

use crate::bind::*;
use alloc::ffi::CString;

/// 信号量封装
pub struct Semaphore {
    handle: rt_sem_handle_t,
    _name: CString,
}

impl Semaphore {
    /// 创建信号量
    pub fn new(name: &str, value: u32) -> RtResult<Self> {
        Self::with_flag(name, value, RT_IPC_FLAG_FIFO)
    }
    
    /// 创建优先级信号量
    pub fn new_prio(name: &str, value: u32) -> RtResult<Self> {
        Self::with_flag(name, value, RT_IPC_FLAG_PRIO)
    }
    
    /// 使用指定标志创建信号量
    pub fn with_flag(name: &str, value: u32, flag: u8) -> RtResult<Self> {
        let name_cstr = CString::new(name)
            .map_err(|_| RtError::InvalidParameter)?;
        
        let handle = unsafe {
            rt_sem_create(name_cstr.as_ptr(), value, flag)
        };
        
        if handle.is_null() {
            return Err(RtError::NoMemory);
        }
        
        Ok(Semaphore {
            handle,
            _name: name_cstr,
        })
    }
    
    /// 获取信号量（阻塞）
    pub fn take(&self) -> RtResult<()> {
        self.take_timeout(None)
    }
    
    /// 尝试获取信号量（非阻塞）
    pub fn try_take(&self) -> RtResult<()> {
        self.take_timeout(Some(0))
    }
    
    /// 带超时的获取信号量
    pub fn take_timeout(&self, timeout_ms: Option<u32>) -> RtResult<()> {
        let timeout = match timeout_ms {
            Some(ms) => ms as i32,
            None => RT_WAITING_FOREVER,
        };
        
        let result = unsafe { rt_sem_take(self.handle, timeout) };
        match result {
            RT_EOK => Ok(()),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 释放信号量
    pub fn release(&self) -> RtResult<()> {
        let result = unsafe { rt_sem_release(self.handle) };
        match result {
            RT_EOK => Ok(()),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 获取信号量句柄
    pub fn handle(&self) -> rt_sem_handle_t {
        self.handle
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            rt_sem_delete(self.handle);
        }
    }
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

/// 二进制信号量（互斥信号量）
pub type BinarySemaphore = Semaphore;

impl BinarySemaphore {
    /// 创建二进制信号量
    pub fn new_binary(name: &str) -> RtResult<Self> {
        Self::new(name, 1)
    }
    
    /// 创建二进制信号量（初始为0）
    pub fn new_binary_empty(name: &str) -> RtResult<Self> {
        Self::new(name, 0)
    }
}
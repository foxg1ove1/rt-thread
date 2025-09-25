// 互斥量封装

use crate::bind::*;
use alloc::ffi::CString;

/// 互斥量封装
pub struct Mutex {
    handle: rt_mutex_handle_t,
    _name: CString,
}

impl Mutex {
    /// 创建互斥量
    pub fn new(name: &str) -> RtResult<Self> {
        Self::with_flag(name, RT_IPC_FLAG_FIFO)
    }
    
    /// 创建优先级互斥量
    pub fn new_prio(name: &str) -> RtResult<Self> {
        Self::with_flag(name, RT_IPC_FLAG_PRIO)
    }
    
    /// 使用指定标志创建互斥量
    pub fn with_flag(name: &str, flag: u8) -> RtResult<Self> {
        let name_cstr = CString::new(name)
            .map_err(|_| RtError::InvalidParameter)?;
        
        let handle = unsafe {
            rt_mutex_create(name_cstr.as_ptr(), flag)
        };
        
        if handle.is_null() {
            return Err(RtError::NoMemory);
        }
        
        Ok(Mutex {
            handle,
            _name: name_cstr,
        })
    }
    
    /// 锁定互斥量（阻塞）
    pub fn lock(&self) -> RtResult<MutexGuard> {
        self.lock_timeout(None)
    }
    
    /// 尝试锁定互斥量（非阻塞）
    pub fn try_lock(&self) -> RtResult<MutexGuard> {
        self.lock_timeout(Some(0))
    }
    
    /// 带超时的锁定互斥量
    pub fn lock_timeout(&self, timeout_ms: Option<u32>) -> RtResult<MutexGuard> {
        let timeout = match timeout_ms {
            Some(ms) => ms as i32,
            None => RT_WAITING_FOREVER,
        };
        
        let result = unsafe { rt_mutex_take(self.handle, timeout) };
        match result {
            RT_EOK => Ok(MutexGuard { mutex: self }),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 获取互斥量句柄
    pub fn handle(&self) -> rt_mutex_handle_t {
        self.handle
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe {
            rt_mutex_delete(self.handle);
        }
    }
}

/// 互斥量守卫，自动释放互斥量
pub struct MutexGuard<'a> {
    mutex: &'a Mutex,
}

impl<'a> Drop for MutexGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            rt_mutex_release(self.mutex.handle);
        }
    }
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

/// 便利宏：在作用域内锁定互斥量
#[macro_export]
macro_rules! mutex_lock {
    ($mutex:expr, $block:block) => {
        {
            let _guard = $mutex.lock()?;
            $block
        }
    };
}
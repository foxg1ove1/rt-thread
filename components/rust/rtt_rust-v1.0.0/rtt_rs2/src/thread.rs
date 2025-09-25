// 线程相关封装

use crate::bind::*;
use alloc::ffi::CString;
use alloc::boxed::Box;
use core::ffi::c_void;

/// 线程句柄封装
pub struct Thread {
    handle: rt_thread_handle_t,
    _name: CString,
}

impl Thread {
    /// 创建新线程
    pub fn new<F>(
        name: &str,
        stack_size: u32,
        priority: u8,
        entry: F,
    ) -> RtResult<Self>
    where
        F: FnOnce() + Send + 'static,
    {
        let name_cstr = CString::new(name)
            .map_err(|_| RtError::InvalidParameter)?;
        
        // 将闭包装箱并转换为原始指针
        let boxed_entry = Box::new(entry);
        let entry_ptr = Box::into_raw(boxed_entry) as *mut c_void;
        
        let handle = unsafe {
            rt_thread_create(
                name_cstr.as_ptr(),
                thread_entry_wrapper::<F>,
                entry_ptr,
                stack_size,
                priority,
                20, // 默认时间片
            )
        };
        
        if handle.is_null() {
            // 释放已分配的内存
            unsafe {
                let _ = Box::from_raw(entry_ptr as *mut F);
            }
            return Err(RtError::NoMemory);
        }
        
        Ok(Thread {
            handle,
            _name: name_cstr,
        })
    }
    
    /// 启动线程
    pub fn start(&self) -> RtResult<()> {
        let result = unsafe { rt_thread_startup(self.handle) };
        RtError::from_rt_err(result).into()
    }
    
    /// 挂起线程
    pub fn suspend(&self) -> RtResult<()> {
        let result = unsafe { rt_thread_suspend(self.handle) };
        RtError::from_rt_err(result).into()
    }
    
    /// 恢复线程
    pub fn resume(&self) -> RtResult<()> {
        let result = unsafe { rt_thread_resume(self.handle) };
        RtError::from_rt_err(result).into()
    }
    
    /// 获取线程句柄
    pub fn handle(&self) -> rt_thread_handle_t {
        self.handle
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            rt_thread_delete(self.handle);
        }
    }
}

// 线程入口包装函数
extern "C" fn thread_entry_wrapper<F>(parameter: *mut c_void)
where
    F: FnOnce() + Send + 'static,
{
    if !parameter.is_null() {
        let entry = unsafe { Box::from_raw(parameter as *mut F) };
        entry();
    }
}

// 线程相关便利函数

/// 获取当前线程
pub fn current_thread() -> rt_thread_handle_t {
    unsafe { rt_thread_self() }
}

/// 让出CPU
pub fn yield_now() {
    unsafe { rt_schedule(); }
}

/// 延时指定毫秒
pub fn delay_ms(ms: u32) {
    unsafe { rt_thread_mdelay(ms as i32); }
}

/// 延时指定tick
pub fn delay_ticks(ticks: u32) {
    unsafe { rt_thread_delay(ticks); }
}

/// 延时指定秒
pub fn delay_s(s: u32) {
    delay_ms(s * 1000);
}

/// 延时指定微秒（近似）
pub fn delay_us(us: u32) {
    if us >= 1000 {
        delay_ms(us / 1000);
    } else if us > 0 {
        delay_ms(1);
    }
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}
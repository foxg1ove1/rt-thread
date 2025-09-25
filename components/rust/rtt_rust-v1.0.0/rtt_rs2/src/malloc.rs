// 内存分配相关功能

use crate::bind::*;
use core::ffi::c_void;

/// 分配内存
pub fn rt_malloc_safe(size: usize) -> Option<*mut u8> {
    if size == 0 {
        return None;
    }
    
    let ptr = unsafe { rt_malloc(size) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr as *mut u8)
    }
}

/// 获取内存使用信息
pub fn get_memory_info() -> (u32, u32, u32) {
    let mut total = 0u32;
    let mut used = 0u32;
    let mut max_used = 0u32;
    
    unsafe {
        rt_memory_info(&mut total, &mut used, &mut max_used);
    }
    
    (total, used, max_used)
}

/// 打印内存使用信息
pub fn print_memory_info() {
    let (total, used, max_used) = get_memory_info();
    
    unsafe {
        rt_kprintf(b"Memory Info:\n\0".as_ptr() as *const i8);
        rt_kprintf(b"  Total: %u bytes\n\0".as_ptr() as *const i8, total);
        rt_kprintf(b"  Used: %u bytes\n\0".as_ptr() as *const i8, used);
        rt_kprintf(b"  Max Used: %u bytes\n\0".as_ptr() as *const i8, max_used);
        rt_kprintf(b"  Free: %u bytes\n\0".as_ptr() as *const i8, total - used);
    }
}


/// 释放内存
pub fn rt_free_safe(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            rt_free(ptr as *mut c_void);
        }
    }
}

/// 重新分配内存
pub fn rt_realloc_safe(ptr: *mut u8, new_size: usize) -> Option<*mut u8> {
    let new_ptr = unsafe { rt_realloc(ptr as *mut c_void, new_size) };
    if new_ptr.is_null() {
        None
    } else {
        Some(new_ptr as *mut u8)
    }
}

/// 分配并清零内存
pub fn rt_calloc_safe(count: usize, size: usize) -> Option<*mut u8> {
    if count == 0 || size == 0 {
        return None;
    }
    
    let ptr = unsafe { rt_calloc(count, size) };
    if ptr.is_null() {
        None
    } else {        
        Some(ptr as *mut u8)
    }
}
#![no_std]
// #![feature(alloc_error_handler)]
#![feature(alloc_error_handler)]
#![feature(allow_internal_unstable)]
#![feature(linkage)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
extern crate alloc;
extern crate libc;
use alloc::vec;
use alloc::ffi::CString;
use cty::c_char;
use cty::c_ulong;
use core::time::Duration;
// use core::sync::atomic::{AtomicU32, Ordering};

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
mod allocator;
mod thread;
pub mod api;
pub mod ffi;
pub mod fmt;
pub mod time;
const REQUEST_RECV: u32 = 0;
// use libc::c_void;
// use libc::size_t;
pub enum RTTError {
    ThreadStartupErr,
    MutexTakeTimeout,
    SemaphoreTakeTimeout,
    QueueSendTimeout,
    QueueReceiveTimeout,
    OutOfMemory,

    DeviceNotFound,
    DeviceOpenFailed,
    DeviceCloseFailed,
    DeviceReadFailed,
    DeviceWriteFailed,
    DeviceTransFailed,
    DeviceConfigFailed,
    DeviceSetRxCallBackFailed,
    DeviceSetTxCallBackFailed,

    FuncUnDefine,
}
pub type RTResult<T> = Result<T, RTTError>;
#[no_mangle]
pub extern "C" fn rust_main() -> u32 {

    // const FMT_STR: &str = "[rust lib]: vec address\n\0";
    const FMT_STR: &str = "rust vec elements: %d, %d, %d\0";
    
    let a = vec![1, 2, 3];
    unsafe {
        // 将静态字符串转换为*const c_char
        let fmt_ptr = FMT_STR.as_ptr() as *const c_char;
        let x: u64 = 4;
        let y: u64 = 3;
        let z = x.wrapping_mul(y);  // 仅测试乘法
        // 调用FFI函数，传递vec中的每个元素（注意类型匹配）
        // i32与C语言的int通常兼容，直接传递即可
        time::sleep(Duration::new(1, 0));
        ffi::rt_kprintf(
            fmt_ptr,
            a[0],  // 第一个元素
            a[1],  // 第二个元素
            z   // 第三个元素
        );
    }

    // unsafe {
    //     let fmt_ptr = FMT_STR.as_ptr() as *const c_char;
        
    //     // 调用 FFI 函数，无堆内存分配
    //     ffi::rt_kprintf(fmt_ptr);
    // } 
    REQUEST_RECV
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // println!("PANIC:\n{}", _info);
    loop {}
}


// #![cfg_attr(not(features = "std"), no_std)] 
// extern crate alloc;
// use alloc::vec;
// use alloc::alloc::{GlobalAlloc, Layout};
// use cty::c_char;
// use core::ptr;
// // 类型定义（根据你的要求）
// pub type stack_t = sigaltstack;
// pub type sig_atomic_t = ::core::ffi::c_int;
// pub type sig_t = _sig_func_ptr;
// pub type rt_bool_t = ::core::ffi::c_int;
// pub type rt_int8_t = ::core::ffi::c_schar;
// pub type rt_int16_t = ::core::ffi::c_short;
// pub type rt_int32_t = ::core::ffi::c_int;
// pub type rt_uint8_t = ::core::ffi::c_uchar;
// pub type rt_uint16_t = ::core::ffi::c_ushort;
// pub type rt_uint32_t = ::core::ffi::c_uint;
// pub type rt_int64_t = ::core::ffi::c_long;
// pub type rt_uint64_t = ::core::ffi::c_ulong;
// pub type rt_base_t = rt_int64_t;
// pub type rt_ubase_t = rt_uint64_t;
// pub type rt_size_t = rt_ubase_t;
// pub type rt_ssize_t = rt_base_t;
// pub type rt_intptr_t = rt_base_t;
// pub type rt_uintptr_t = rt_ubase_t;
// pub type rt_err_t = rt_base_t;
// pub type rt_time_t = rt_uint32_t;
// pub type rt_tick_t = rt_uint32_t;
// pub type rt_flag_t = rt_base_t;
// pub type rt_dev_t = rt_ubase_t;
// pub type rt_off_t = rt_base_t;
// pub type rt_atomic_t = rt_base_t;

// // 声明未定义的类型（根据实际环境补充）
// pub struct sigaltstack;
// pub type _sig_func_ptr = unsafe extern "C" fn(rt_int32_t) -> rt_int32_t;

// // 全局分配器（假设已按之前的正确实现）
// #[global_allocator]
// pub static GLOBAL_ALLOC: RtAllocator = RtAllocator;

// // 全局分配器实现（简化版，确保alloc/dealloc正常）
// pub struct RtAllocator;
// unsafe impl GlobalAlloc for RtAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         // 调用底层rt_malloc_align（已验证正常）
//         let ptr = crate::ffi::rt_malloc_align(layout.size(), layout.align());
//         ptr as *mut u8
//     }
//     unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
//         crate::ffi::rt_free_align(ptr as *mut core::ffi::c_void);
//     }
// }

// // FFI函数声明（假设已正确实现）
// pub mod ffi {
//     use super::*;
//     extern "C" {
//         pub fn rt_malloc_align(size: usize, align: usize) -> *mut core::ffi::c_void;
//         pub fn rt_free_align(ptr: *mut core::ffi::c_void);
//         pub fn rt_kprintf(fmt: *const c_char, ...);
//     }
// }

// /// 手动调用全局分配器创建Vec的函数
// /// - `capacity`: Vec的初始容量（可容纳的元素数量）
// /// - 返回值：成功时返回Vec，失败时返回None
// unsafe fn create_vec_manually<T: Default>(capacity: usize) -> Option<Vec<T>> {
//     // 步骤1：函数入口
//     let step1 = "[CREATE_VEC] 进入create_vec_manually - 容量: %d\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step1, capacity);

//     // 步骤2：元素大小和对齐
//     let elem_size = core::mem::size_of::<T>();
//     let elem_align = core::mem::align_of::<T>();
//     let step2 = "[CREATE_VEC] 元素信息 - 大小: %d, 对齐: %d\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step2, elem_size, elem_align);

//     // 步骤2.5：验证输入值
//     let step2_5 = "[CREATE_VEC] 准备计算总大小 - capacity: %d, elem_size: %d\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step2_5, capacity, elem_size);

//     // 步骤3：手动实现溢出检查（替代checked_mul）
//     let step3_start = "[CREATE_VEC] 开始手动溢出检查...\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step3_start);

//     let total_size = if elem_size == 0 {
//         // 特殊情况：元素大小为0（如零大小类型）
//         0
//     } else if capacity > usize::MAX / elem_size {
//         // 溢出检查：如果capacity > 最大可容纳值 → 溢出
//         let err = "[CREATE_VEC] 错误：容量×大小溢出（手动检查）\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(err);
//         return None;
//     } else {
//         // 无溢出，直接计算
//         let val = capacity * elem_size;
//         let ok = "[CREATE_VEC] 手动检查无溢出，总大小: %d\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(ok, val);
//         val
//     };

//     // 步骤4：创建内存布局
//     let layout = match Layout::from_size_align(total_size, elem_align) {
//         Ok(val) => val,
//         Err(_) => {
//             let err = "[CREATE_VEC] 错误：创建Layout失败\n\0"
//                 .as_ptr() as *const c_char;
//             ffi::rt_kprintf(err);
//             return None;
//         }
//     };
//     let step4 = "[CREATE_VEC] Layout创建成功\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step4);

//     // 步骤5：调用全局分配器
//     let data_ptr = GLOBAL_ALLOC.alloc(layout);
//     if data_ptr.is_null() {
//         let err = "[CREATE_VEC] 错误：分配器返回空指针\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(err);
//         return None;
//     }
//     let step5 = "[CREATE_VEC] 内存分配成功 - 原始地址: %p\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step5, data_ptr);

//     // 步骤6：转换指针类型
//     let elem_ptr = data_ptr as *mut T;
//     let step6 = "[CREATE_VEC] 指针转换完成 - 元素指针: %p\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step6, elem_ptr);

//     // 步骤7：初始化内存（逐索引调试）
//     let step7_start = "[CREATE_VEC] 开始初始化内存（共%d个元素）\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step7_start, capacity);
    
//     // 计算内存范围（再次确认）
//     let elem_size = core::mem::size_of::<T>();
//     let alloc_start_addr = elem_ptr as usize;
//     let alloc_end_addr = alloc_start_addr + capacity * elem_size;
//     let addr_range = "[CREATE_VEC] 分配范围: 0x%x 至 0x%x（共%d字节）\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(addr_range, alloc_start_addr, alloc_end_addr - 1, alloc_end_addr - alloc_start_addr);

//     // 改用while循环（避免for循环的迭代器可能带来的问题）
//     let mut i = 0;
//     for i in 0..capacity {
//         // 1. 打印当前索引和计算的地址
//         let current_ptr = elem_ptr.add(i);
//         let current_addr = current_ptr as usize;
//         let loop_start = "[CREATE_VEC] 初始化索引%d - 地址: 0x%x（检查是否在分配范围内）\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(loop_start, i, current_addr);

//         // 2. 验证地址是否在分配的内存范围内（防越界）
//         if current_addr < alloc_start_addr || current_addr >= alloc_end_addr {
//             let addr_err = "[CREATE_VEC] 错误：索引%d的地址0x%x超出分配范围！\n\0"
//                 .as_ptr() as *const c_char;
//             ffi::rt_kprintf(addr_err, i, current_addr);
//             return None;
//         }

//         // 3. 尝试获取默认值（单独打印，验证T::default()是否正常）
//         let default_val = T::default();
//         let default_ok = "[CREATE_VEC] 索引%d：获取默认值成功\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(default_ok, i);

//         // 4. 执行写入操作（分步骤打印）
//         let write_start = "[CREATE_VEC] 索引%d：开始执行ptr::write\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(write_start, i);

//         ptr::write(current_ptr, default_val);

//         let write_ok = "[CREATE_VEC] 索引%d：ptr::write执行完成\n\0"
//             .as_ptr() as *const c_char;
//         ffi::rt_kprintf(write_ok, i);
//     }
    
//     let step7_end = "[CREATE_VEC] 内存初始化完成\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step7_end);

//     let step7_end = "[CREATE_VEC] 内存初始化完成\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step7_end);

//     // 步骤8：构造Vec
//     let vec = Vec::from_raw_parts(elem_ptr, 0, capacity);
//     let step8 = "[CREATE_VEC] Vec构造完成 - 长度: %d, 容量: %d\n\0"
//         .as_ptr() as *const c_char;
//     ffi::rt_kprintf(step8, vec.len(), vec.capacity());

//     Some(vec)
// }

// #[no_mangle]
// pub extern "C" fn rust_main() -> u32 {
//     let a = vec![1, 2, 3];
//     0
// }

// // panic处理函数
// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     unsafe {
//         let panic_msg = "[rust panic]: 发生panic！\n\0".as_ptr() as *const c_char;
//         ffi::rt_kprintf(panic_msg);
//     }
//     loop {}
// }
    
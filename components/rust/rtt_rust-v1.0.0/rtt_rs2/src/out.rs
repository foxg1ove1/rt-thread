// 输出相关功能

use crate::bind::*;
use core::fmt::Write;

/// 栈上的字符串缓冲区，避免堆分配
pub struct StackBuffer {
    buffer: [u8; 512],
    pos: usize,
}

impl StackBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; 512],
            pos: 0,
        }
    }
    
    pub fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let bytes = s.as_bytes();
        let remaining = self.buffer.len().saturating_sub(self.pos + 1);
        let copy_len = bytes.len().min(remaining);
        
        if copy_len > 0 {
            self.buffer[self.pos..self.pos + copy_len].copy_from_slice(&bytes[..copy_len]);
            self.pos += copy_len;
        }
        
        Ok(())
    }
    
    pub fn as_c_str(&mut self) -> *const i8 {
        if self.pos < self.buffer.len() {
            self.buffer[self.pos] = 0;
        } else {
            self.buffer[self.buffer.len() - 1] = 0;
        }
        self.buffer.as_ptr() as *const i8
    }
    
    pub fn clear(&mut self) {
        self.pos = 0;
        self.buffer[0] = 0;
    }
    
    pub fn len(&self) -> usize {
        self.pos
    }
    
    pub fn is_empty(&self) -> bool {
        self.pos == 0
    }
}

impl Write for StackBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s)
    }
}

impl Default for StackBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// 安全的打印函数
pub fn safe_print(s: &str) {
    if s.is_empty() {
        return;
    }
    
    // 限制字符串长度以避免栈溢出
    let safe_str = if s.len() > 400 {
        &s[..400]
    } else {
        s
    };
    
    unsafe {
        let bytes = safe_str.as_bytes();
        let mut buffer = [0u8; 512];
        let len = bytes.len().min(511);
        
        buffer[..len].copy_from_slice(&bytes[..len]);
        buffer[len] = 0;
        
        rt_kprintf(buffer.as_ptr() as *const i8);
    }
}

/// 打印一行
pub fn safe_println(s: &str) {
    safe_print(s);
    safe_print("\n");
}
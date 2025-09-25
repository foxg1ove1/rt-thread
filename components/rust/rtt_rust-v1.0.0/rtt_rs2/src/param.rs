// 命令行参数处理

use alloc::vec::Vec;
use alloc::string::String;
use core::ffi::{CStr, c_char};

/// 命令行参数封装
pub struct Param {
    args: Vec<String>,
    current: usize,
}

impl Param {
    /// 从C风格参数创建
    pub fn from_raw(argc: i32, argv: *const *const c_char) -> Self {
        let mut args = Vec::new();
        
        if argc > 0 && !argv.is_null() {
            for i in 0..argc {
                unsafe {
                    let arg_ptr = *argv.add(i as usize);
                    if !arg_ptr.is_null() {
                        if let Ok(c_str) = CStr::from_ptr(arg_ptr).to_str() {
                            args.push(c_str.to_string());
                        }
                    }
                }
            }
        }
        
        Param { args, current: 0 }
    }
    
    /// 创建空参数
    pub fn empty() -> Self {
        Param {
            args: Vec::new(),
            current: 0,
        }
    }
    
    /// 从字符串数组创建
    pub fn from_strings(strings: &[&str]) -> Self {
        let args = strings.iter().map(|s| s.to_string()).collect();
        Param { args, current: 0 }
    }
    
    /// 获取参数数量
    pub fn len(&self) -> usize {
        self.args.len()
    }
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
    
    /// 获取指定位置的参数
    pub fn get(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(|s| s.as_str())
    }
    
    /// 获取第一个参数（通常是命令名）
    pub fn command(&self) -> Option<&str> {
        self.get(0)
    }
    
    /// 获取所有参数的引用
    pub fn all(&self) -> &[String] {
        &self.args
    }
    
    /// 检查是否包含某个参数
    pub fn contains(&self, arg: &str) -> bool {
        self.args.iter().any(|a| a == arg)
    }
    
    /// 查找参数的位置
    pub fn find(&self, arg: &str) -> Option<usize> {
        self.args.iter().position(|a| a == arg)
    }
    
    /// 获取选项参数的值（形如 --key=value 或 --key value）
    pub fn get_option(&self, key: &str) -> Option<&str> {
        // 查找 --key=value 形式
        let key_eq = format!("{}=", key);
        for arg in &self.args {
            if arg.starts_with(&key_eq) {
                return Some(&arg[key_eq.len()..]);
            }
        }
        
        // 查找 --key value 形式
        if let Some(pos) = self.find(key) {
            if pos + 1 < self.args.len() {
                return Some(&self.args[pos + 1]);
            }
        }
        
        None
    }
    
    /// 检查是否有指定的标志参数
    pub fn has_flag(&self, flag: &str) -> bool {
        self.contains(flag)
    }
}

impl Iterator for Param {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.args.len() {
            let result = self.args[self.current].clone();
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Param {
    fn len(&self) -> usize {
        self.args.len() - self.current
    }
}

impl Clone for Param {
    fn clone(&self) -> Self {
        Param {
            args: self.args.clone(),
            current: 0, // 重置迭代器位置
        }
    }
}
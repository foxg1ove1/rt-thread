// 消息队列封装

use crate::bind::*;
use alloc::ffi::CString;
use alloc::vec::Vec;
use core::mem;
use core::ffi::c_void;

/// 消息队列封装
pub struct MessageQueue {
    handle: rt_mq_handle_t,
    _name: CString,
    msg_size: usize,
}

impl MessageQueue {
    /// 创建消息队列
    pub fn new(name: &str, msg_size: usize, max_msgs: usize) -> RtResult<Self> {
        Self::with_flag(name, msg_size, max_msgs, RT_IPC_FLAG_FIFO)
    }
    
    /// 创建优先级消息队列
    pub fn new_prio(name: &str, msg_size: usize, max_msgs: usize) -> RtResult<Self> {
        Self::with_flag(name, msg_size, max_msgs, RT_IPC_FLAG_PRIO)
    }
    
    /// 使用指定标志创建消息队列
    pub fn with_flag(
        name: &str,
        msg_size: usize,
        max_msgs: usize,
        flag: u8,
    ) -> RtResult<Self> {
        let name_cstr = CString::new(name)
            .map_err(|_| RtError::InvalidParameter)?;
        
        let handle = unsafe {
            rt_mq_create(
                name_cstr.as_ptr(),
                msg_size,
                max_msgs,
                flag,
            )
        };
        
        if handle.is_null() {
            return Err(RtError::NoMemory);
        }
        
        Ok(MessageQueue {
            handle,
            _name: name_cstr,
            msg_size,
        })
    }
    
    /// 发送消息（结构化数据）
    pub fn send<T>(&self, msg: &T) -> RtResult<()>
    where
        T: Copy,
    {
        let size = mem::size_of::<T>();
        if size > self.msg_size {
            return Err(RtError::InvalidParameter);
        }
        
        let result = unsafe {
            rt_mq_send(
                self.handle,
                msg as *const T as *const c_void,
                size,
            )
        };
        
        match result {
            RT_EOK => Ok(()),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 发送字节数据
    pub fn send_bytes(&self, data: &[u8]) -> RtResult<()> {
        if data.len() > self.msg_size {
            return Err(RtError::InvalidParameter);
        }
        
        let result = unsafe {
            rt_mq_send(
                self.handle,
                data.as_ptr() as *const c_void,
                data.len(),
            )
        };
        
        match result {
            RT_EOK => Ok(()),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 接收消息（结构化数据）
    pub fn recv<T>(&self) -> RtResult<T>
    where
        T: Copy + Default,
    {
        self.recv_timeout(None)
    }
    
    /// 尝试接收消息（非阻塞）
    pub fn try_recv<T>(&self) -> RtResult<T>
    where
        T: Copy + Default,
    {
        self.recv_timeout(Some(0))
    }
    
    /// 带超时的接收消息
    pub fn recv_timeout<T>(&self, timeout_ms: Option<u32>) -> RtResult<T>
    where
        T: Copy + Default,
    {
        let timeout = match timeout_ms {
            Some(ms) => ms as i32,
            None => RT_WAITING_FOREVER,
        };
        
        let mut msg: T = T::default();
        let size = mem::size_of::<T>();
        
        if size > self.msg_size {
            return Err(RtError::InvalidParameter);
        }
        
        let result = unsafe {
            rt_mq_recv(
                self.handle,
                &mut msg as *mut T as *mut c_void,
                size,
                timeout,
            )
        };
        
        match result {
            RT_EOK => Ok(msg),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 接收字节数据
    pub fn recv_bytes(&self) -> RtResult<Vec<u8>> {
        self.recv_bytes_timeout(None)
    }
    
    /// 尝试接收字节数据（非阻塞）
    pub fn try_recv_bytes(&self) -> RtResult<Vec<u8>> {
        self.recv_bytes_timeout(Some(0))
    }
    
    /// 带超时的接收字节数据
    pub fn recv_bytes_timeout(&self, timeout_ms: Option<u32>) -> RtResult<Vec<u8>> {
        let timeout = match timeout_ms {
            Some(ms) => ms as i32,
            None => RT_WAITING_FOREVER,
        };
        
        let mut buffer = vec![0u8; self.msg_size];
        
        let result = unsafe {
            rt_mq_recv(
                self.handle,
                buffer.as_mut_ptr() as *mut c_void,
                self.msg_size,
                timeout,
            )
        };
        
        match result {
            RT_EOK => Ok(buffer),
            _ => Err(RtError::from_rt_err(result)),
        }
    }
    
    /// 获取消息大小
    pub fn msg_size(&self) -> usize {
        self.msg_size
    }
    
    /// 获取队列句柄
    pub fn handle(&self) -> rt_mq_handle_t {
        self.handle
    }
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        unsafe {
            rt_mq_delete(self.handle);
        }
    }
}

unsafe impl Send for MessageQueue {}
unsafe impl Sync for MessageQueue {}
// RT-Thread C API bindings

// 包含生成的绑定
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// 重新导出常用常量
pub const RT_EOK: rt_err_t = 0;
pub const RT_ERROR: rt_err_t = 1;
pub const RT_ETIMEOUT: rt_err_t = 2;
pub const RT_EFULL: rt_err_t = 3;
pub const RT_EEMPTY: rt_err_t = 4;
pub const RT_ENOMEM: rt_err_t = 5;
pub const RT_ENOSYS: rt_err_t = 6;
pub const RT_EBUSY: rt_err_t = 7;
pub const RT_EIO: rt_err_t = 8;
pub const RT_EINTR: rt_err_t = 9;
pub const RT_EINVAL: rt_err_t = 10;

pub const RT_WAITING_FOREVER: rt_int32_t = -1;
pub const RT_IPC_FLAG_FIFO: rt_uint8_t = 0x00;
pub const RT_IPC_FLAG_PRIO: rt_uint8_t = 0x01;

// 错误类型转换
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtError {
    Ok,
    Error,
    Timeout,
    Full,
    Empty,
    NoMemory,
    NoSys,
    Busy,
    Io,
    Intr,
    InvalidParameter,
}

impl RtError {
    pub fn from_rt_err(err: rt_err_t) -> Self {
        match err {
            RT_EOK => RtError::Ok,
            RT_ERROR => RtError::Error,
            RT_ETIMEOUT => RtError::Timeout,
            RT_EFULL => RtError::Full,
            RT_EEMPTY => RtError::Empty,
            RT_ENOMEM => RtError::NoMemory,
            RT_ENOSYS => RtError::NoSys,
            RT_EBUSY => RtError::Busy,
            RT_EIO => RtError::Io,
            RT_EINTR => RtError::Intr,
            RT_EINVAL => RtError::InvalidParameter,
            _ => RtError::Error,
        }
    }
    
    pub fn is_ok(&self) -> bool {
        matches!(self, RtError::Ok)
    }
    
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl From<rt_err_t> for RtError {
    fn from(err: rt_err_t) -> Self {
        RtError::from_rt_err(err)
    }
}

// Result type alias
pub type RtResult<T> = Result<T, RtError>;
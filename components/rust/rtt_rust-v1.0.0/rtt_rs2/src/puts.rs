// 打印宏定义

use crate::out::{StackBuffer, safe_print};
use core::fmt::Write;

/// 格式化打印宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut buffer = $crate::out::StackBuffer::new();
            let _ = write!(buffer, $($arg)*);
            $crate::out::safe_print(
                core::str::from_utf8(&buffer.buffer[..buffer.pos]).unwrap_or("???")
            );
        }
    };
}

/// 格式化打印一行宏
#[macro_export]
macro_rules! println {
    () => {
        $crate::out::safe_print("\n")
    };
    ($($arg:tt)*) => {
        {
            $crate::print!($($arg)*);
            $crate::out::safe_print("\n");
        }
    };
}

/// 调试打印宏（仅在debug模式下有效）
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::print!("[DEBUG] ");
            $crate::print!($($arg)*);
            $crate::print!("\n");
        }
    };
}

/// 错误打印宏
#[macro_export]
macro_rules! error_print {
    ($($arg:tt)*) => {
        {
            $crate::print!("[ERROR] ");
            $crate::print!($($arg)*);
            $crate::print!("\n");
        }
    };
}

/// 警告打印宏
#[macro_export]
macro_rules! warn_print {
    ($($arg:tt)*) => {
        {
            $crate::print!("[WARN] ");
            $crate::print!($($arg)*);
            $crate::print!("\n");
        }
    };
}

/// 信息打印宏
#[macro_export]
macro_rules! info_print {
    ($($arg:tt)*) => {
        {
            $crate::print!("[INFO] ");
            $crate::print!($($arg)*);
            $crate::print!("\n");
        }
    };
}
// no_std 环境下的常用导入

pub use core::{
    mem,
    ptr,
    slice,
    str,
    fmt,
    convert::{TryFrom, TryInto, From, Into},
    option::{Option, Option::*},
    result::{Result as CoreResult, Result::*},
    marker::{Send, Sync, Copy, Clone},
    cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering},
    ops::{Drop, Deref, DerefMut},
    default::Default,
    iter::{Iterator, IntoIterator},
};

pub use alloc::{
    vec,
    vec::Vec,
    string::{String, ToString},
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format,
};
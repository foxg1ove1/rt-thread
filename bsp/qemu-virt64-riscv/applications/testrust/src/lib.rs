#![no_std]

extern crate alloc;

use alloc::string::String;
use rtt_main::rtt_main;
use rtt_rs2::param::Param;
use rtt_rs2::println;

// appname: 应用的名字，在命令中将被使用
// run: 是否使用rt-thread操作系统的自动执行功能
// cmd: 是否添加app到命令行
// desc: 命令行程序的描述
// 最简版本：#[rtt_main(appname="demo")]
//  请自行调用函数 __demo_main_func
#[rtt_main(appname="demo", run=true, cmd=true, desc="demo app.")]
fn main(param: Param) {
    for i in param {
        println!("{}", String::from_utf8_lossy(&*i))
    }
}
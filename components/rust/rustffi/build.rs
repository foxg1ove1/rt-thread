use std::env;
use std::path::PathBuf;

fn main() {
    // 获取 rt-thread 根目录的路径
    let rtthread_path = env::var("RTT_ROOT")
        .unwrap_or_else(|_| "/opt/rtt/rt-thread".to_string());
    // 获取硬件目标 BSP 的路径
    // let hw_target_path = env::var("HW_TARGET")
    //     .unwrap_or_else(|_| format!("{}/bsp/qemu-virt64-riscv", rtthread_path));
    let cpu_path =
        // 指定CPU Path
        env::var("CPU_PATH").unwrap_or_else(|_| format!("{}/libcpu/risc-v/common64", rtthread_path));

    // riscv64-unknown-elf-gcc 的 include 路径，需要下载源码编译
    let cross_path = env::var("CROSS_INCLUDE").unwrap_or_else(|_| {
        // "/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/arm-none-eabi/include".to_string()
        "/opt/riscv/riscv64-unknown-elf/include".to_string()
    });

    // 如果 header 文件发生变化，重新生成绑定
    println!(
        "cargo:rerun-if-changed={}/include/rtthread.h",
        rtthread_path
    );
    println!(
        "cargo:rerun-if-changed={}/components/libc/posix/libdl/dlfcn.h",
        rtthread_path
    );    
    // 构造 bindgen 构建器
    let bindings = bindgen::Builder::default()
        // 指定主头文件
        .header(format!("{}/include/rtthread.h", rtthread_path))
        .header(format!("{}/components/libc/posix/libdl/dlfcn.h", rtthread_path))
        .header(format!("{}/components/libc/posix/libdl/dlelf.h", rtthread_path))
        .header(format!("{}/components/libc/posix/libdl/dlmodule.h", rtthread_path))
        // 使用 core 而非 std
        .use_core()
        // 合并 extern "C" 块
        .merge_extern_blocks(true)
        // 不在 enum 前面添加前缀
        .prepend_enum_name(false)
        // 关闭布局测试
        .layout_tests(false)
        
        // 添加额外的 clang 参数（include 路径）  
        // TODO:rust没有提供这个平台，后续可能需要专门构建一个平台：https://doc.rust-lang.net.cn/stable/rustc/targets/custom.html
        .clang_arg("--target=riscv64-unknown-elf")

        // bsp的配置头文件：rtconfig.h
        // .clang_arg(format!("-I{}", hw_target_path))
        .clang_arg(format!("-I/home/fox/OSPP/RT-Thread/qemu-edu/machines/qemu-virt-riscv64"))

        // CPU 相关的头文件路径
        .clang_arg(format!("-I{}", cpu_path))
        .clang_arg(format!("-I/home/fox/OSPP/riscv_rtt_rust/rt-thread/libcpu/risc-v/virt64"))

        // rt-thread 的头文件路径
        .clang_arg(format!("-I{}/include", rtthread_path))
        .clang_arg(format!("-I{}/components/finsh", rtthread_path))
        .clang_arg(format!("-I{}/components/libc/posix/libdl", rtthread_path))
        // .clang_arg(format!("-I{}/components/legacy", rtthread_path))
        // .clang_arg(format!("-I{}/components/drivers/include", rtthread_path))
        // .clang_arg(format!("-I/opt/riscv/riscv64-unknown-elf/include"))

        .clang_arg(format!("-I{}", cross_path))
        .clang_arg(format!("-I/opt/riscv/riscv64-unknown-elf/include"))

        .clang_arg("-march=rv64imac")
        .clang_arg("-mabi=lp64")
        // 生成绑定
        .generate()
        .expect("Unable to generate bindings");

    // 将生成的绑定写入到 $OUT_DIR/bindings.rs 中
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write ffi!");
}

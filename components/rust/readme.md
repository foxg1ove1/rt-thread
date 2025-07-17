# 使用rust在qemu-virt64-riscv平台串口输出

## 环境依赖

本项目构建十分依赖工具链。由于rust和clang提供的构建平台不能完全一致匹配，所以需要手动统一工具链配置,重点需匹配：`--with-abi=lp64 --with-arch=rv64imac`。

```bash
clang target：riscv64-unknown-elf
rustc target: riscv64imac-unknown-none-elf
```

## 项目结构

```bash
.
├── readme.md
└── rustffi         
    ├── Cargo.lock
    ├── Cargo.toml      // rust构建配置文件
    ├── build.rs        // 配置绑定rt-thread头文件
    ├── rust.h
    └── src
        ├── ffi.rs      // bindgen自动绑定的C头文件
        └── lib.rs      
```

## 编译程序

```bash
➜  qemu-virt-riscv64 ✗ pwd
~/home/fox/OSPP/RT-Thread~/qemu-edu/machines/qemu-virt-riscv64
➜  qemu-virt-riscv64 ✗ scons -j6
......
➜  qemu-virt-riscv64 ✗ ./run.sh
```

## 输出示例

```bash
 \ | /
- RT -     Thread Smart Operating System
 / | \     5.2.0 build Jul 10 2025 14:48:44
 2006 - 2024 Copyright by RT-Thread team
lwIP-2.0.3 initialized!
[I/sal.skt] Socket Abstraction Layer initialize success.
[I/utest] utest is initialize success.
[I/utest] total utest testcase num: (1)
[I/drivers.serial] Using /dev/ttyS0 as default console
[rust-rt_kprintf]: hello world
Hello RISC-V
```
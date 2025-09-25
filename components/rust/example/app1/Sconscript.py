Import('rtconfig')
from building import *
import os
import subprocess

cwd = GetCurrentDir()

# 设置Rust编译环境变量
rust_env = os.environ.copy()
rust_env['RTT_PATH'] = Dir('#').abspath

# Rust编译函数
def build_rust_lib(target, source, env):
    print("Building Rust application...")
    os.chdir(cwd)
    
    # 执行cargo build
    result = subprocess.run([
        'cargo', 'build', 
        '--target', 'riscv64gc-unknown-none-elf',
        '--release'
    ], env=rust_env)
    
    if result.returncode != 0:
        raise Exception("Rust compilation failed")
    
    print("Rust build completed successfully")

# 创建Rust库目标
rust_target = os.path.join(cwd, 'target/riscv64gc-unknown-none-elf/release/librust_example.a')

# 注册构建任务
rust_lib = env.Command(
    rust_target,
    Glob('src/*.rs') + ['Cargo.toml'],  # 依赖源文件
    build_rust_lib
)

# 设置库路径和库名
LIBPATH = [os.path.join(cwd, 'target/riscv64gc-unknown-none-elf/release')]
LIBS = ['rust_example']

# 返回构建结果
Return('LIBPATH', 'LIBS')
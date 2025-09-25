#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import subprocess
import json
import sys
from pathlib import Path

class RustBuilder:
    def __init__(self, rust_dir):
        self.rust_dir = Path(rust_dir)
        self.rtt_path = os.environ.get('RTT_PATH')
        if not self.rtt_path:
            raise Exception("RTT_PATH environment variable not set")
        
        self.target = "riscv64gc-unknown-none-elf"
        self.setup_environment()
    
    def setup_environment(self):
        """设置Rust构建环境"""
        self.env = os.environ.copy()
        self.env['RTT_PATH'] = self.rtt_path
        self.env['CARGO_TARGET_DIR'] = str(self.rust_dir / 'target')
        
        # 检查Rust工具链
        try:
            result = subprocess.run(['rustc', '--version'], 
                                  capture_output=True, text=True)
            print("Rust version:", result.stdout.strip())
        except FileNotFoundError:
            raise Exception("Rust not found. Please install Rust toolchain.")
        
        # 检查目标平台
        try:
            result = subprocess.run(['rustup', 'target', 'list', '--installed'], 
                                  capture_output=True, text=True)
            if self.target not in result.stdout:
                print(f"Installing target {self.target}...")
                subprocess.run(['rustup', 'target', 'add', self.target], 
                             check=True)
        except Exception as e:
            print(f"Warning: Could not check/install target: {e}")
    
    def build_core_libraries(self):
        """构建核心Rust库"""
        core_path = self.rust_dir / 'rtt_rust-v1.0.0'
        
        # 构建rtt_rs2
        rtt_rs2_path = core_path / 'rtt_rs2'
        if not self._build_crate(rtt_rs2_path, "rtt_rs2"):
            return False
        
        # 构建rtt_main  
        rtt_main_path = core_path / 'rtt_main'
        if not self._build_crate(rtt_main_path, "rtt_main"):
            return False
        
        return True
    
    def build_applications(self):
        """构建所有应用"""
        apps_dir = self.rust_dir / 'applications'
        if not apps_dir.exists():
            print("No applications directory found")
            return []
        
        built_apps = []
        for app_dir in apps_dir.iterdir():
            if app_dir.is_dir() and (app_dir / 'Cargo.toml').exists():
                if self._build_application(app_dir):
                    built_apps.append({
                        'name': app_dir.name,
                        'libpath': str(self.rust_dir / 'target' / self.target / 'release'),
                        'libs': [f'lib{app_dir.name}.a']
                    })
        
        return built_apps
    
    def _build_crate(self, crate_path, crate_name):
        """构建单个crate"""
        if not crate_path.exists():
            print(f"Crate path not found: {crate_path}")
            return False
        
        print(f"Building {crate_name}...")
        
        try:
            result = subprocess.run([
                'cargo', 'build',
                '--target', self.target,
                '--release'
            ], cwd=crate_path, env=self.env, capture_output=True, text=True)
            
            if result.returncode != 0:
                print(f"Failed to build {crate_name}:")
                print("STDOUT:", result.stdout)
                print("STDERR:", result.stderr)
                return False
            
            print(f"Successfully built {crate_name}")
            return True
            
        except Exception as e:
            print(f"Exception building {crate_name}: {e}")
            return False
    
    def _build_application(self, app_path):
        """构建单个应用"""
        app_name = app_path.name
        print(f"Building application: {app_name}")
        
        # 检查依赖
        cargo_toml = app_path / 'Cargo.toml'
        if not self._check_dependencies(cargo_toml):
            print(f"Dependencies check failed for {app_name}")
            return False
        
        return self._build_crate(app_path, app_name)
    
    def _check_dependencies(self, cargo_toml_path):
        """检查Cargo.toml依赖配置"""
        try:
            with open(cargo_toml_path, 'r') as f:
                content = f.read()
                # 简单检查是否包含必要的依赖
                if 'rtt_main' not in content or 'rtt_rs2' not in content:
                    print("Warning: Missing required dependencies in", cargo_toml_path)
                    return False
                return True
        except Exception as e:
            print(f"Error checking dependencies: {e}")
            return False
    
    def clean(self):
        """清理构建产物"""
        target_dir = self.rust_dir / 'target'
        if target_dir.exists():
            import shutil
            shutil.rmtree(target_dir)
            print("Cleaned target directory")

if __name__ == "__main__":
    # 直接运行脚本进行测试
    if len(sys.argv) > 1 and sys.argv[1] == "clean":
        builder = RustBuilder(os.path.dirname(os.path.abspath(__file__)))
        builder.clean()
    else:
        builder = RustBuilder(os.path.dirname(os.path.abspath(__file__)))
        if builder.build_core_libraries():
            apps = builder.build_applications()
            print(f"Built {len(apps)} applications successfully")
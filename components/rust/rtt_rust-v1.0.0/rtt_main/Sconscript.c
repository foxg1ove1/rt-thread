Import('rtconfig')
from building import *
import os

cwd = GetCurrentDir()

# 添加Rust应用支持
if GetDepend(['PKG_USING_RTT_RUST']):
    # 添加applications目录下的Rust应用
    rust_apps = []
    apps_dir = os.path.join(cwd, 'applications')
    
    if os.path.exists(apps_dir):
        for app_name in os.listdir(apps_dir):
            app_path = os.path.join(apps_dir, app_name)
            sconscript_path = os.path.join(app_path, 'SConscript')
            
            if os.path.isdir(app_path) and os.path.exists(sconscript_path):
                print("Found Rust application: {}".format(app_name))
                rust_apps.append(app_path)
    
    # 构建每个Rust应用
    group = []
    for app_path in rust_apps:
        try:
            result = SConscript(os.path.join(app_path, 'SConscript'))
            if result:
                if isinstance(result, tuple) and len(result) >= 2:
                    LIBPATH, LIBS = result[:2]
                    group.append({'LIBPATH': LIBPATH, 'LIBS': LIBS})
                print("Successfully built Rust app in: {}".format(app_path))
        except Exception as e:
            print("Failed to build Rust app in {}: {}".format(app_path, str(e)))
    
    Return('group')

Return([])
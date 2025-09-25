Import('RTT_ROOT')
Import('rtconfig')
from building import *

# change it if you want to use a different chip
llvm_target = 'riscv64imac-unknown-none-elf'

cargo = Builder(action = [
        'cargo build --manifest-path ${SOURCE.abspath} --target ${LLVM_TARGET}  --target-dir ${TARGET.dir.abspath}',
        Copy('${TARGET.abspath}', '${TARGET.dir.abspath}/${LLVM_TARGET}/debug/${TARGET.file}')
    ],
    suffix = '.a',
    src_suffix = '.toml',
    prefix = 'lib',
    chdir = 0)

Env.Append(BUILDERS = {'Cargo' : cargo})
Env.AppendUnique(LLVM_TARGET = llvm_target)

cwd = GetCurrentDir()
src = Glob('*.c')
CPPPATH = [cwd, ]

# 'rustffi' is ".a" file name
rustffi = Env.Cargo('rustffi', 'Cargo.toml')
Env.AlwaysBuild(rustffi)

group = DefineGroup('rustffi', src, depend = [''], LIBS = [rustffi], CPPPATH = CPPPATH, #LINKFLAGS = ' -latomic',  #LINKFLAGS = ' -z muldefs'
                    )


Return('group')

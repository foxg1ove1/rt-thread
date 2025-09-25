use std::env;
use std::path::PathBuf;

fn main() {
    let rtt_path = env::var("RTT_PATH")
        .unwrap_or_else(|_| {
            println!("cargo:warning=RTT_PATH not set, using default");
            "/opt/rtt/rt-thread".to_string()
        });
    
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=RTT_PATH");
    println!("cargo:rustc-link-lib=rtthread");
    
    let include_paths = vec![
        format!("{}/include", rtt_path),
        format!("{}/components/finsh", rtt_path),
        format!("{}/components/dfs", rtt_path),
        format!("{}/components/drivers/include", rtt_path),
        format!("{}/libcpu/risc-v/virt64", rtt_path),
        format!("{}/libcpu/risc-v/common", rtt_path),
        format!("{}/../machines/qemu-virt-riscv64", rtt_path),
        format!("/opt/riscv64-unknown-elf-toolchain-10.2.0-2020.12.8-x86_64-linux-ubuntu14/riscv64-unknown-elf/include"),
    ];
    
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(true)
        .size_t_is_usize(true)
        .use_core()
        .ctypes_prefix("libc")
        .allowlist_function("rt_.*")
        .allowlist_function("msh_.*")
        .allowlist_type("rt_.*")
        .allowlist_var("RT_.*")
        .generate_comments(false)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    
    for include_path in include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path));
    }
    
    // RISC-V specific defines
    builder = builder
        .clang_arg("-DARCH_CPU_64BIT")
        .clang_arg("-D__riscv")
        .clang_arg("-D__riscv_xlen=64");
    
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
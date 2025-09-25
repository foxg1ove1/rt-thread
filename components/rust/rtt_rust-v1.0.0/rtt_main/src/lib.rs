// RT-Thread 应用程序入口宏

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta, Meta, Lit};

/// RT-Thread应用程序入口宏
/// 
/// # 参数
/// - `appname`: 应用程序名称
/// - `run`: 是否自动运行 (可选, 默认false)
/// - `cmd`: 是否注册为MSH命令 (可选, 默认false) 
/// - `desc`: 命令描述 (可选)
/// 
/// # 示例
/// ```rust
/// #[rtt_main(appname="test", cmd=true, desc="Test application")]
/// fn main(param: Param) {
///     println!("Hello from Rust!");
/// }
/// ```
#[proc_macro_attribute]
pub fn rtt_main(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let mut appname = String::new();
    let mut run = false;
    let mut cmd = false;
    let mut desc = String::new();
    
    // 解析属性参数
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("appname") => {
                if let Lit::Str(lit) = nv.lit {
                    appname = lit.value();
                }
            }
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("run") => {
                if let Lit::Bool(lit) = nv.lit {
                    run = lit.value;
                }
            }
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("cmd") => {
                if let Lit::Bool(lit) = nv.lit {
                    cmd = lit.value;
                }
            }
            NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("desc") => {
                if let Lit::Str(lit) = nv.lit {
                    desc = lit.value();
                }
            }
            _ => {}
        }
    }
    
    if appname.is_empty() {
        panic!("appname is required");
    }
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_body = &input_fn.block;
    
    let main_func_name = format!("__{}_main_func", appname);
    let main_func_ident = syn::Ident::new(&main_func_name, proc_macro2::Span::call_site());
    
    let mut output = quote! {
        #fn_vis fn #fn_name(param: rtt_rs2::param::Param) #fn_body
    };
    
    if cmd {
        // 注册为MSH命令
        let cmd_func_name = format!("__{}_cmd_func", appname);
        let cmd_func_ident = syn::Ident::new(&cmd_func_name, proc_macro2::Span::call_site());
        
        output.extend(quote! {
            #[no_mangle]
            pub extern "C" fn #cmd_func_ident(argc: i32, argv: *const *const i8) -> i32 {
                let param = rtt_rs2::param::Param::from_raw(argc, argv);
                #fn_name(param);
                0
            }
            
            #[used]
            #[link_section = ".rti_fn.6"]
            static #main_func_ident: extern "C" fn() = || {
                extern "C" {
                    fn msh_cmd_register(
                        name: *const i8, 
                        func: extern "C" fn(i32, *const *const i8) -> i32, 
                        desc: *const i8
                    ) -> i32;
                }
                unsafe {
                    msh_cmd_register(
                        concat!(#appname, "\0").as_ptr() as *const i8,
                        #cmd_func_ident,
                        concat!(#desc, "\0").as_ptr() as *const i8
                    );
                }
            };
        });
    } else {
        // 普通函数入口
        output.extend(quote! {
            #[no_mangle]
            pub extern "C" fn #main_func_ident() {
                let param = rtt_rs2::param::Param::empty();
                #fn_name(param);
            }
        });
    }
    
    if run {
        // 自动运行
        let auto_run_name = format!("__{}_auto_run", appname);
        let auto_run_ident = syn::Ident::new(&auto_run_name, proc_macro2::Span::call_site());
        
        output.extend(quote! {
            #[used]
            #[link_section = ".rti_fn.6"]
            static #auto_run_ident: extern "C" fn() = || {
                extern "C" {
                    fn rt_thread_create(
                        name: *const i8,
                        entry: extern "C" fn(*mut core::ffi::c_void),
                        parameter: *mut core::ffi::c_void,
                        stack_size: u32,
                        priority: u8,
                        tick: u32
                    ) -> *mut core::ffi::c_void;
                    fn rt_thread_startup(thread: *mut core::ffi::c_void) -> i32;
                }
                
                extern "C" fn thread_wrapper(_: *mut core::ffi::c_void) {
                    #main_func_ident();
                }
                
                unsafe {
                    let thread = rt_thread_create(
                        concat!(#appname, "_auto\0").as_ptr() as *const i8,
                        thread_wrapper,
                        core::ptr::null_mut(),
                        2048,
                        10,
                        20
                    );
                    if !thread.is_null() {
                        rt_thread_startup(thread);
                    }
                }
            };
        });
    }
    
    output.into()
}
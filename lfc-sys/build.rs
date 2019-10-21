use std::env;
use std::path::PathBuf;
use cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("longfi-core")
                 .define("BUILD_TESTING", "OFF")
                 .define("CMAKE_C_COMPILER_WORKS", "1")
                 .define("CMAKE_CXX_COMPILER_WORKS", "1")
                 .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=blake2");
    println!("cargo:rustc-link-lib=static=cursor");
    println!("cargo:rustc-link-lib=static=cursor_varint");
    println!("cargo:rustc-link-lib=static=golay");
    println!("cargo:rustc-link-lib=static=lfc");

   // make the bindings
   let bindings = bindgen::Builder::default()
       .raw_line("use cty;")
       .use_core()
       .ctypes_prefix("cty")
       .detect_include_paths(true)
       .header("longfi-core/include/lfc/lfc.h")
       .header("longfi-core/include/lfc/priv/lfc_dg_ser.h")
       .header("longfi-core/include/lfc/priv/lfc_dg_des.h")
       .clang_arg(format!("-I{}/include",dst.display()))
       .whitelist_type("lfc")
       .whitelist_type("lfc_user_cfg")
       .whitelist_type("cursor")
       .whitelist_function("lfc_dg__des")
       .trust_clang_mangling(false)
       .rustfmt_bindings(true)
       .derive_copy(false)
       .derive_debug(false)
       .layout_tests(false)
       .generate()
       .expect("Failed to build LongFi Core Bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


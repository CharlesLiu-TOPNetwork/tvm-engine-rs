fn main() {
    #[cfg(feature = "build_as_xtop_lib")]
    let out_dir = std::env::var("RUST_COMPILE_C_OUTPUT_DIR").unwrap();

    #[cfg(not(feature = "build_as_xtop_lib"))]
    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=../tvm-c-api/");
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .include("../tvm-c-api/")
        .file("../tvm-c-api/tvm_import_instance.cpp")
        .file("../tvm-c-api/protobuf_types/pbasic.pb.cc")
        .file("../tvm-c-api/protobuf_types/pparameters.pb.cc")
        .out_dir(&out_dir)
        .cpp_link_stdlib("stdc++")
        .compile("libtvm-c-api.a");

    println!("cargo:rustc-link-lib=static=tvm-c-api");
}

fn main() {
    println!("cargo:rerun-if-changed=../tvm-c-api/");
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .include("../tvm-c-api/")
        .file("../tvm-c-api/tvm_import_instance.cpp")
        .file("../tvm-c-api/protobuf_types/pbasic.pb.cc")
        .file("../tvm-c-api/protobuf_types/pparameters.pb.cc")
        .cpp_link_stdlib("stdc++")
        .compile("libtvm-c-api.a");

    println!("cargo:rustc-link-lib=static=tvm-c-api")
}

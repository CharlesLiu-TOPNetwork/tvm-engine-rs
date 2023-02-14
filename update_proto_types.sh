#!/bin/bash

# https://docs.rs/protobuf-codegen/latest/protobuf_codegen/
#
# apt-get install protobuf-compiler
# cargo uninstall protobuf-codegen
# cargo install protobuf-codegen

protoc --rust_out ./tvm-engine-types/src/proto/ ./protobuf_types/pbasic.proto
protoc --rust_out ./tvm-engine-types/src/proto/ ./protobuf_types/pparameters.proto
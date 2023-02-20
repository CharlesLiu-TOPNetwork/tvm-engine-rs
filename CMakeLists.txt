add_compile_options(-Wpedantic)

message("============== TOP-Chain Virtual Machine CMAKE ==============")

# find cargo
execute_process(
    COMMAND bash -c "which cargo | grep 'cargo' | tr -d '\n'"
    OUTPUT_VARIABLE CARGO_DIR
)
message(STATUS "Cargo dir: " ${CARGO_DIR})

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD ${CARGO_DIR} build)
    set(TARGET_DIR "debug")
else()
    set(CARGO_CMD ${CARGO_DIR} build --release)
    set(TARGET_DIR "release")
endif()

message(STATUS "Cargo cmd: " ${CARGO_CMD})
set(TVM_LIB_A "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libtvm_engine.a")
message(STATUS "TVM_LIB_A: " ${TVM_LIB_A})
message(STATUS "CMAKE_CURRENT_BINARY_DIR: " ${CMAKE_CURRENT_BINARY_DIR})
message(STATUS "CMAKE_CURRENT_SOURCE_DIR: " ${CMAKE_CURRENT_SOURCE_DIR})
message(STATUS "LIBRARY_OUTPUT_PATH: " ${LIBRARY_OUTPUT_PATH})

add_custom_target(tvm_engine ALL
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMAND cp -f ${TVM_LIB_A} ${LIBRARY_OUTPUT_PATH})

set_target_properties(tvm_engine PROPERTIES LOCATION ${LIBRARY_OUTPUT_PATH})

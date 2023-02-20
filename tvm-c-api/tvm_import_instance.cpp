#include "./tvm_import_instance.h"

#include <cassert>

namespace top {
namespace tvm {

void tvm_import_instance::add_logic(std::shared_ptr<tvm_logic_face> logic) {
    auto current_thread_id_hash = std::to_string(std::hash<std::thread::id>{}(std::this_thread::get_id()));
    std::unique_lock<std::mutex> lock(m_rw_mutex);
    m_tvm_logic_dict.insert({current_thread_id_hash, logic});
}

void tvm_import_instance::remove_logic() {
    auto current_thread_id_hash = std::to_string(std::hash<std::thread::id>{}(std::this_thread::get_id()));
    std::unique_lock<std::mutex> lock(m_rw_mutex);
    assert(m_tvm_logic_dict.find(current_thread_id_hash) != m_tvm_logic_dict.end());
    m_tvm_logic_dict.erase(current_thread_id_hash);
}

std::shared_ptr<tvm_logic_face> tvm_import_instance::current_logic() {
    auto current_thread_id_hash = std::to_string(std::hash<std::thread::id>{}(std::this_thread::get_id()));
    // std::shared_lock<std::mutex> lock(m_rw_mutex); // since C++14
    std::unique_lock<std::mutex> lock(m_rw_mutex);
    assert(m_tvm_logic_dict.find(current_thread_id_hash) != m_tvm_logic_dict.end());
    return m_tvm_logic_dict.at(current_thread_id_hash);
}

void tvm_import_instance::tvm_read_register(uint64_t register_id, uint64_t ptr) {
    return current_logic()->read_register(register_id, ptr);
}
uint64_t tvm_import_instance::tvm_register_len(uint64_t register_id) {
    return current_logic()->register_len(register_id);
}
void tvm_import_instance::tvm_input(uint64_t register_id) {
    return current_logic()->input(register_id);
}
void tvm_import_instance::tvm_result(uint64_t value_len, uint64_t value_ptr) {
    return current_logic()->result(value_len, value_ptr);
}
uint64_t tvm_import_instance::tvm_storage_write(uint64_t key_len, uint64_t key_ptr, uint64_t value_len, uint64_t value_ptr, uint64_t register_id) {
    return current_logic()->storage_write(key_len, key_ptr, value_len, value_ptr, register_id);
}
uint64_t tvm_import_instance::tvm_storage_read(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) {
    return current_logic()->storage_read(key_len, key_ptr, register_id);
}
uint64_t tvm_import_instance::tvm_storage_remove(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) {
    return current_logic()->storage_remove(key_len, key_ptr, register_id);
}
uint64_t tvm_import_instance::tvm_gas_price() {
    return current_logic()->gas_price();
}
void tvm_import_instance::tvm_origin_address(uint64_t register_id) {
    return current_logic()->origin_address(register_id);
}
uint64_t tvm_import_instance::tvm_block_height() {
    return current_logic()->block_height();
}
void tvm_import_instance::tvm_block_coinbase(uint64_t register_id) {
    return current_logic()->block_coinbase(register_id);
}
uint64_t tvm_import_instance::tvm_block_timestamp() {
    return current_logic()->block_timestamp();
}
uint64_t tvm_import_instance::tvm_chain_id() {
    return current_logic()->chain_id();
}
void tvm_import_instance::tvm_log_utf8(uint64_t len, uint64_t ptr) {
    return current_logic()->log_utf8(len, ptr);
}

}  // namespace tvm

}  // namespace top

/// RUST CALL C
extern "C" {
using top::tvm::tvm_import_instance;
// register common
void tvm_read_register(uint64_t register_id, uint64_t ptr) {
    return tvm_import_instance::instance()->tvm_read_register(register_id, ptr);
}
uint64_t tvm_register_len(uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_register_len(register_id);
}

// io input && output
void tvm_input(uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_input(register_id);
}
void tvm_result(uint64_t value_len, uint64_t value_ptr) {
    return tvm_import_instance::instance()->tvm_result(value_len, value_ptr);
}

// io storage
uint64_t tvm_storage_write(uint64_t key_len, uint64_t key_ptr, uint64_t value_len, uint64_t value_ptr, uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_storage_write(key_len, key_ptr, value_len, value_ptr, register_id);
}
uint64_t tvm_storage_read(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_storage_read(key_len, key_ptr, register_id);
}
uint64_t tvm_storage_remove(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_storage_remove(key_len, key_ptr, register_id);
}

// env
uint64_t tvm_gas_price() {
    return tvm_import_instance::instance()->tvm_gas_price();
}
void tvm_origin_address(uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_origin_address(register_id);
}
uint64_t tvm_block_height() {
    return tvm_import_instance::instance()->tvm_block_height();
}
void tvm_block_coinbase(uint64_t register_id) {
    return tvm_import_instance::instance()->tvm_block_coinbase(register_id);
}
uint64_t tvm_block_timestamp() {
    return tvm_import_instance::instance()->tvm_block_timestamp();
}
uint64_t tvm_chain_id() {
    return tvm_import_instance::instance()->tvm_chain_id();
}

// logs
void tvm_log_utf8(uint64_t len, uint64_t ptr) {
    return tvm_import_instance::instance()->tvm_log_utf8(len, ptr);
}
}

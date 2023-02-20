#pragma once

#include "./tvm_logic_face.h"

#include <map>
#include <memory>
#include <mutex>
#include <string>
#include <thread>

namespace top {
namespace tvm {
class tvm_import_instance {
public:
    static tvm_import_instance * instance();

    std::map<std::string, std::shared_ptr<tvm_logic_face>> m_tvm_logic_dict;
    std::mutex m_rw_mutex;

public:
    void add_logic(std::shared_ptr<tvm_logic_face> logic);
    void remove_logic();
    std::shared_ptr<tvm_logic_face> current_logic();

public:
    /// RUST CALL C
    void tvm_read_register(uint64_t register_id, uint64_t ptr);
    uint64_t tvm_register_len(uint64_t register_id);
    void tvm_input(uint64_t register_id);
    void tvm_result(uint64_t value_len, uint64_t value_ptr);
    uint64_t tvm_storage_write(uint64_t key_len, uint64_t key_ptr, uint64_t value_len, uint64_t value_ptr, uint64_t register_id);
    uint64_t tvm_storage_read(uint64_t key_len, uint64_t key_ptr, uint64_t register_id);
    uint64_t tvm_storage_remove(uint64_t key_len, uint64_t key_ptr, uint64_t register_id);
    uint64_t tvm_gas_price();
    void tvm_origin_address(uint64_t register_id);
    uint64_t tvm_block_height();
    void tvm_block_coinbase(uint64_t register_id);
    uint64_t tvm_block_timestamp();
    uint64_t tvm_chain_id();
    void tvm_log_utf8(uint64_t len, uint64_t ptr);

private:
    tvm_import_instance() {
    }
    ~tvm_import_instance() {
    }
};
}  // namespace tvm
}  // namespace top
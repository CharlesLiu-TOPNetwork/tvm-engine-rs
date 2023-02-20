#pragma once

#include <cstdint>

namespace top {
namespace tvm {

class tvm_logic_face {
public:
public:
    /// RUST CALL C
    virtual void read_register(uint64_t register_id, uint64_t ptr) = 0;
    virtual uint64_t register_len(uint64_t register_id) = 0;
    virtual void input(uint64_t register_id) = 0;
    virtual void result(uint64_t value_len, uint64_t value_ptr) = 0;
    virtual uint64_t storage_write(uint64_t key_len, uint64_t key_ptr, uint64_t value_len, uint64_t value_ptr, uint64_t register_id) = 0;
    virtual uint64_t storage_read(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) = 0;
    virtual uint64_t storage_remove(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) = 0;
    virtual uint64_t gas_price() = 0;
    virtual void origin_address(uint64_t register_id) = 0;
    virtual uint64_t block_height() = 0;
    virtual void block_coinbase(uint64_t register_id) = 0;
    virtual uint64_t block_timestamp() = 0;
    virtual uint64_t chain_id() = 0;
    virtual void log_utf8(uint64_t len, uint64_t ptr) = 0;
};
}  // namespace tvm

}  // namespace top

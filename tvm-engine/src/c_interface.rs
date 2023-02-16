mod interface {

    use protobuf::Message;
    use tvm_engine_runtime::{
        io::{StorageIntermediate, IO},
        runtime::Runtime,
    };
    use tvm_engine_types::{PCallArgs, PReturnResult};

    use crate::{engine::Engine, types::EngineInterfaceExpect};

    #[no_mangle]
    pub extern "C" fn call() -> bool {
        let rt = Runtime;
        let mut engine = Engine::new(rt, &rt);
        let input = rt.get_input().to_vec();
        let args = PCallArgs::parse_from_bytes(&input).engine_interface_expect("Err CallArgs Deserialize");
        let (bytes, b) = match engine.call(args.into()) {
            Ok(r) => {
                let bytes =
                    PReturnResult::write_to_bytes(&r.into()).engine_interface_expect("Err ReturnResult Serialize");
                (bytes, true)
            }
            Err(err) => {
                let r = PReturnResult {
                    status: u32::MAX,
                    status_data: err.kind.as_bytes().to_vec(),
                    gas_used: err.gas_used,
                    ..Default::default()
                };
                let bytes = PReturnResult::write_to_bytes(&r).engine_interface_expect("Err ReturnResult Serialize");
                (bytes, false)
            }
        };
        rt.set_output(&bytes);
        b
    }
}

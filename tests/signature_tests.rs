use ring::{signature, test};

<<<<<<< HEAD
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
=======
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
>>>>>>> 0a1fec88e (set wasi-sdk compiler + archiver)

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn signature_impl_test() {
    test::compile_time_assert_clone::<signature::Signature>();
    test::compile_time_assert_copy::<signature::Signature>();
    test::compile_time_assert_send::<signature::Signature>();
    test::compile_time_assert_sync::<signature::Signature>();
}

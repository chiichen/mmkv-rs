use autocxx::prelude::*;

include_cpp! {
    #include "MMKV.h"
    extern_cpp_type!("std::string_view", crate::base::StringView<'a>)
    safety!(unsafe_ffi)
    generate!("mmkv::MMKV") // allowlist a function
    pod!("std::string_view")
}
pub mod bindings {
    include!(concat!(
        std::env!("OUT_DIR"),
        "/autocxx-build-dir/rs/autocxx-ffi-default-gen.rs"
    ));
}

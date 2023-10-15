#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![feature(cfg_match)]
#![feature(type_alias_impl_trait)]

pub mod fs;
pub mod io;
pub mod task;

#[inline]
pub fn current_runtime() -> &'static impl aral_trait::Runtime {
    cfg_match! {
        cfg(feature = "runtime-tokio") => {
            use aral_runtime_tokio::TokioRuntime as R;
        }
        _ => {
            use aral_runtime_noop::NoopRuntime as R;
        }
    }
    &R
}

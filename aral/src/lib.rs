#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![feature(cfg_match)]

pub mod fs;
pub mod io;
pub mod task;

use aral_trait::{fs::Fs, task::Task};
use std::sync::OnceLock;

fn get_runtime() -> &'static impl aral_trait::Runtime {
    cfg_match! {
        cfg(feature = "adapter-tokio") => {
            use aral_adapter_tokio::Runtime;
        }
        _ => {
            use aral_adapter_noop::Runtime;
        }
    }
    &Runtime
}

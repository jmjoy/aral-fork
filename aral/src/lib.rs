#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

pub mod io;
pub mod fs;
pub mod task;

use std::sync::OnceLock;
use aral_trait::{fs::Fs, task::Task};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub enum Runtime {
    #[cfg(feature = "adapter-tokio")]
    Tokio,
}

impl Default for Runtime {
    #[inline]
    fn default() -> Self {
        if cfg!(feature = "adapter-tokio") {
            Runtime::Tokio
        } else {
            panic!("please specify runtime");
        }
    }
}

impl aral_trait::Runtime for Runtime {
    #[inline]
    fn fs(&'static self) -> &'static impl Fs {
        match self {
            #[cfg(feature = "adapter-tokio")]
            Runtime::Tokio => aral_adapter_tokio::RUNTIME.fs(),
            _ => panic!("please specify runtime"),
        }
    }

    #[inline]
    fn task(&'static self) -> &'static impl Task {
        match self {
            #[cfg(feature = "adapter-tokio")]
            Runtime::Tokio => aral_adapter_tokio::RUNTIME.task(),
            _ => panic!("please specify runtime"),
        }
    }
}

pub fn set_runtime(runtime: Runtime) {
    if let Err(_) = RUNTIME.set(runtime) {
        panic!("runtime has set")
    }
}

fn get_runtime() -> &'static impl aral_trait::Runtime {
    RUNTIME.get_or_init(Default::default)
}

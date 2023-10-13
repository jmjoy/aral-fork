#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

mod fs;
mod task;

pub struct Runtime;

impl aral_trait::Runtime for Runtime {
    fn name() -> &'static str {
        "tokio"
    }
}

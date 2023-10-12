#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

use aral_trait::{fs::Fs, task::Task, Runtime};

pub mod fs;
pub mod task;

pub static RUNTIME: RuntimeImpl = RuntimeImpl;

pub struct RuntimeImpl;

impl Runtime for RuntimeImpl {
    #[inline]
    fn fs(&'static self) -> &'static impl Fs {
        &fs::FS
    }

    #[inline]
    fn task(&'static self) -> &'static impl Task {
        &task::TASK
    }
}

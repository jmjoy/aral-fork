#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

pub mod fs;
pub mod io;
pub mod task;

pub trait Runtime {
    fn fs(&'static self) -> &'static impl fs::Fs;
    fn task(&'static self) -> &'static impl task::Task;
}

#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

pub mod fs;
pub mod io;
pub mod task;
// pub mod net;

pub trait Runtime {
    fn name(&self) -> &'static str;

    fn fs(&self) -> &'static impl fs::Fs;

    fn task(&self) -> &'static impl task::Task;
}

#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

pub mod fs;
pub mod io;
pub mod task;

pub trait Runtime: fs::Fs + task::Task {}

#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![allow(unreachable_code)]

use std::marker::PhantomData;

macro_rules! no_adapter_specified {
    () => {
        panic!("no adapter specified, please enable one of `adapter-*` features");
    };
}

mod fs;
mod task;

pub struct Runtime;

#[derive(Default)]
struct Noop;

struct NoopData<T>(PhantomData<T>);

impl aral_trait::Runtime for Runtime {
    fn name() -> &'static str {
        "noop"
    }
}

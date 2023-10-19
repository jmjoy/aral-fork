#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![allow(unreachable_code)]

use aral_trait::Runtime;
use std::marker::PhantomData;

macro_rules! no_runtime_specified {
    () => {
        panic!("no runtime specified, please enable one of `runtime-*` features for crate aral");
    };
}

mod fs;
mod net;
mod task;

pub struct NoopRuntime;

#[derive(Default)]
struct Noop;

struct NoopData<T>(PhantomData<T>);

impl Runtime for NoopRuntime {
    #[inline]
    fn name(&self) -> &'static str {
        "noop"
    }

    #[inline]
    fn fs(&self) -> &'static impl aral_trait::fs::Fs {
        &Self
    }

    #[inline]
    fn task(&self) -> &'static impl aral_trait::task::Task {
        &Self
    }

    #[inline]
    fn net(&self) -> &'static impl aral_trait::net::Net {
        &Self
    }
}

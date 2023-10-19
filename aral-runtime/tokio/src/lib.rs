#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

use crate::{fs::TokioFs, task::TokioTask};
use aral_trait::{fs::Fs, net::Net, task::Task, Runtime};
use net::TokioNet;

mod fs;
mod net;
mod task;

pub struct TokioRuntime;

impl Runtime for TokioRuntime {
    #[inline]
    fn name(&self) -> &'static str {
        "tokio"
    }

    #[inline]
    fn fs(&self) -> &'static impl Fs {
        &TokioFs
    }

    #[inline]
    fn task(&self) -> &'static impl Task {
        &TokioTask
    }

    #[inline]
    fn net(&self) -> &'static impl Net {
        &TokioNet
    }
}

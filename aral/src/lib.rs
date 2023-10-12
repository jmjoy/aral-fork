#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

use enum_dispatch::enum_dispatch;
use fs::Fs;
use io::Read;
use std::{default, ptr::null_mut, sync::OnceLock};

pub mod fs;
pub mod io;

pub trait Runtime {
    fn fs(&'static self) -> &'static impl fs::Fs;
}

pub struct TokioRuntime;

pub struct TokioFs;

pub struct TokioFile;

static TOKIO_RUNTIME: TokioRuntime = TokioRuntime;
static TOKIO_FS: TokioFs = TokioFs;

impl Runtime for TokioRuntime {
    fn fs(&'static self) -> &'static impl fs::Fs {
        &TOKIO_FS
    }
}

enum WhichRuntime {
    TokioRuntime,
}

impl Runtime for WhichRuntime {
    fn fs(&'static self) -> &'static impl fs::Fs {
        match self {
            WhichRuntime::TokioRuntime => Runtime::fs(&TOKIO_RUNTIME),
        }
    }
}

static WHICH_RUNTIME: OnceLock<WhichRuntime> = OnceLock::new();

fn set_which_runtime(rt: WhichRuntime) {
    WHICH_RUNTIME.set(rt);
}

fn get_which_runtime() -> &'static WhichRuntime {
    WHICH_RUNTIME.get().unwrap()
}

async fn foo() {
    let mut file = get_which_runtime().fs().create_file("path").await.unwrap();
    file.read(&mut []).await.unwrap();
}

use crate::{TokioFile, TokioFs, TokioRuntime};
use std::{
    fs::{Metadata, Permissions},
    future::{self, Future},
    io::Result,
    path::{Path, PathBuf},
};

pub trait Fs {
    fn create_file(
        &self, path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<impl FileTrait>> + Send;

    // fn open_file(
    //     path: impl AsRef<Path>,
    // ) -> impl std::future::Future<Output = Result<impl FileTrait>> + Send;

    // fn canonicalize(path: impl AsRef<Path>) -> impl Future<Output =
    // Result<PathBuf>> + Send;

    // fn copy(
    //     &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    // ) -> impl Future<Output = Result<u64>> + Send;

    // fn create_dir(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<()>> + Send;

    // fn create_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<()>> + Send;

    // fn hard_link(
    //     &self, src: impl AsRef<Path>, dst: impl AsRef<Path>,
    // ) -> impl Future<Output = Result<()>> + Send;

    // fn metadata(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<Metadata>> + Send;

    // fn read(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<Vec<u8>>> + Send;

    // fn read_link(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<PathBuf>> + Send;

    // fn read_to_string(&self, path: impl AsRef<Path>)
    //     -> impl Future<Output = Result<String>> + Send;

    // fn remove_dir(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<()>> + Send;

    // fn remove_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<()>> + Send;

    // fn remove_file(&self, path: impl AsRef<Path>) -> impl Future<Output =
    // Result<()>> + Send;

    // fn rename(
    //     &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    // ) -> impl Future<Output = Result<()>> + Send;

    // fn set_permissions(
    //     &self, path: impl AsRef<Path>, perm: Permissions,
    // ) -> impl Future<Output = Result<()>> + Send;

    // fn symlink_metadata(
    //     &self, path: impl AsRef<Path>,
    // ) -> impl Future<Output = Result<Metadata>> + Send;

    // fn write(
    //     &self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>,
    // ) -> impl Future<Output = Result<()>> + Send;
}

impl Fs for TokioFs {
    fn create_file(
        &self, path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<impl FileTrait>> + Send {
        async { Ok(TokioFile) }
    }
}

pub trait FileTrait: crate::io::Read + Send + Sync {}

impl FileTrait for TokioFile {}

use crate::io::{BufRead, Read, Seek, Write};
use std::{
    fs::{Metadata, Permissions},
    future::Future,
    io::Result,
    path::{Path, PathBuf},
};

pub trait File: Read + Write + Seek {
    fn metadata(&self) -> impl Future<Output = Result<Metadata>> + Send;

    fn set_len(&self, size: u64) -> impl Future<Output = Result<()>> + Send;

    fn set_permissions(&self, perm: Permissions) -> impl Future<Output = Result<()>> + Send;

    fn sync_all(&self) -> impl Future<Output = Result<()>> + Send;

    fn sync_data(&self) -> impl Future<Output = Result<()>> + Send;

    fn try_clone(&self) -> impl Future<Output = Result<impl File>> + Send;
}

pub trait OpenOptions: Default {
    fn append(&mut self, append: bool) -> &mut Self;

    fn create(&mut self, create: bool) -> &mut Self;

    fn create_new(&mut self, create_new: bool) -> &mut Self;

    fn open(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<impl File>> + Send;

    fn read(&mut self, read: bool) -> &mut Self;

    fn truncate(&mut self, truncate: bool) -> &mut Self;

    fn write(&mut self, write: bool) -> &mut Self;
}

pub trait Fs {
    fn create_file(&self, path: impl AsRef<Path> + Send)
        -> impl Future<Output = Result<impl File>> + Send;

    fn open_file(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<impl File>> + Send;

    fn open_options(&self) -> impl OpenOptions;

    fn canonicalize(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<PathBuf>> + Send;

    fn copy(
        &self, from: impl AsRef<Path> + Send, to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = Result<u64>> + Send;

    fn create_dir(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<()>> + Send;

    fn create_dir_all(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<()>> + Send;

    fn hard_link(
        &self, src: impl AsRef<Path> + Send, dst: impl AsRef<Path> + Send,
    ) -> impl Future<Output = Result<()>> + Send;

    fn metadata(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<Metadata>> + Send;

    fn read(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<Vec<u8>>> + Send;

    fn read_link(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<PathBuf>> + Send;

    fn read_to_string(&self, path: impl AsRef<Path> + Send)
        -> impl Future<Output = Result<String>> + Send;

    fn remove_dir(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<()>> + Send;

    fn remove_dir_all(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<()>> + Send;

    fn remove_file(&self, path: impl AsRef<Path> + Send) -> impl Future<Output = Result<()>> + Send;

    fn rename(
        &self, from: impl AsRef<Path> + Send, to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = Result<()>> + Send;

    fn set_permissions(
        &self, path: impl AsRef<Path> + Send, perm: Permissions,
    ) -> impl Future<Output = Result<()>> + Send;

    fn symlink_metadata(
        &self, path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = Result<Metadata>> + Send;

    fn write(
        &self, path: impl AsRef<Path> + Send, contents: impl AsRef<[u8]> + Send,
    ) -> impl Future<Output = Result<()>> + Send;
}

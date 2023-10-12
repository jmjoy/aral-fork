use crate::io::{BufRead, Read, Seek, Write};
use std::{
    fs::{Metadata, Permissions},
    future::Future,
    io::Result,
    path::{Path, PathBuf},
};

pub trait File: Read + Write + Seek {
    fn metadata(&self) -> impl Future<Output = Result<Metadata>>;

    fn set_len(&self, size: u64) -> impl Future<Output = Result<()>>;

    fn set_permissions(&self, perm: Permissions) -> impl Future<Output = Result<()>>;

    fn sync_all(&self) -> impl Future<Output = Result<()>>;

    fn sync_data(&self) -> impl Future<Output = Result<()>>;

    fn try_clone(&self) -> impl Future<Output = Result<impl File>>;
}

pub trait OpenOptions: Default {
    fn append(&mut self, append: bool) -> &mut Self;

    fn create(&mut self, create: bool) -> &mut Self;

    fn create_new(&mut self, create_new: bool) -> &mut Self;

    fn open(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>>;

    fn read(&mut self, read: bool) -> &mut Self;

    fn truncate(&mut self, truncate: bool) -> &mut Self;

    fn write(&mut self, write: bool) -> &mut Self;
}

pub trait Fs {
    fn create_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>>;

    fn open_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>>;

    fn open_options(&self) -> impl OpenOptions;

    fn canonicalize(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>>;

    fn copy(
        &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    ) -> impl Future<Output = Result<u64>>;

    fn create_dir(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    fn create_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    fn hard_link(
        &self, src: impl AsRef<Path>, dst: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>>;

    fn metadata(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>>;

    fn read(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>>>;

    fn read_link(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>>;

    fn read_to_string(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<String>>;

    fn remove_dir(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    fn remove_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    fn remove_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    fn rename(
        &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>>;

    fn set_permissions(
        &self, path: impl AsRef<Path>, perm: Permissions,
    ) -> impl Future<Output = Result<()>>;

    fn symlink_metadata(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>>;

    fn write(
        &self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = Result<()>>;
}

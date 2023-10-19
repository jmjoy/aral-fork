use crate::Runtime;
use crate::current_runtime;
use std::{
    fs::{Metadata, Permissions},
    io::Result,
    path::{Path, PathBuf}, future::Future,
};
use crate::io::{Read, Seek, Write};

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

    fn open(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File + '_>>;

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

#[inline]
pub async fn create_file(path: impl AsRef<Path>) -> Result<impl File> {
    current_runtime().fs().create_file(path).await
}

#[inline]
pub async fn open_file(path: impl AsRef<Path>) -> Result<impl File> {
    current_runtime().fs().open_file(path).await
}

#[inline]
pub fn open_options() -> impl OpenOptions {
    current_runtime().fs().open_options()
}

#[inline]
pub async fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    current_runtime().fs().canonicalize(path).await
}

#[inline]
pub async fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
    current_runtime().fs().copy(from, to).await
}

#[inline]
pub async fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().create_dir(path).await
}

#[inline]
pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().create_dir_all(path).await
}

#[inline]
pub async fn hard_link(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().hard_link(src, dst).await
}

#[inline]
pub async fn metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    current_runtime().fs().metadata(path).await
}

#[inline]
pub async fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    current_runtime().fs().read(path).await
}

#[inline]
pub async fn read_link(path: impl AsRef<Path>) -> Result<PathBuf> {
    current_runtime().fs().read_link(path).await
}

#[inline]
pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    current_runtime().fs().read_to_string(path).await
}

#[inline]
pub async fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().remove_dir(path).await
}

#[inline]
pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().remove_dir_all(path).await
}

#[inline]
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().remove_file(path).await
}

#[inline]
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    current_runtime().fs().rename(from, to).await
}

#[inline]
pub async fn set_permissions(path: impl AsRef<Path>, perm: Permissions) -> Result<()> {
    current_runtime().fs().set_permissions(path, perm).await
}

#[inline]
pub async fn symlink_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    current_runtime().fs().symlink_metadata(path).await
}

#[inline]
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    current_runtime().fs().write(path, contents).await
}

use crate::current_runtime;
pub use aral_trait::fs::{File, OpenOptions};
use aral_trait::{fs::Fs, Runtime};
use std::{
    fs::{Metadata, Permissions},
    io::Result,
    path::{Path, PathBuf},
};

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

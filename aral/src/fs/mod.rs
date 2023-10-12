use std::{
    fs::{Metadata, Permissions},
    io::{Result, SeekFrom},
    path::{Path, PathBuf},
};
use crate::get_runtime;
use aral_trait::{Runtime, fs::Fs};

#[inline]
pub async fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    get_runtime().fs().canonicalize(path).await
}

#[inline]
pub async fn copy(from: impl AsRef<Path> , to: impl AsRef<Path> ) -> Result<u64> {
    get_runtime().fs().copy(from, to).await
}

#[inline]
pub async fn create_dir(path: impl AsRef<Path> ) -> Result<()> {
    get_runtime().fs().create_dir(path).await
}

#[inline]
pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    get_runtime().fs().create_dir_all(path).await
}

#[inline]
pub async fn hard_link(src: impl AsRef<Path> , dst: impl AsRef<Path> ) -> Result<()> {
    get_runtime().fs().hard_link(src, dst).await
}

#[inline]
pub async fn metadata(path: impl AsRef<Path> ) -> Result<Metadata> {
    get_runtime().fs().metadata(path).await
}

#[inline]
pub async fn read(path: impl AsRef<Path> ) -> Result<Vec<u8>> {
    get_runtime().fs().read(path).await
}

#[inline]
pub async fn read_link(path: impl AsRef<Path>) -> Result<PathBuf> {
    get_runtime().fs().read_link(path).await
}

#[inline]
pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    get_runtime().fs().read_to_string(path).await
}

#[inline]
pub async fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    get_runtime().fs().remove_dir(path).await
}

#[inline]
pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    get_runtime().fs().remove_dir_all(path).await
}

#[inline]
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    get_runtime().fs().remove_file(path).await
}

#[inline]
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    get_runtime().fs().rename(from, to).await
}

#[inline]
pub async fn set_permissions(path: impl AsRef<Path>, perm: Permissions) -> Result<()> {
    get_runtime().fs().set_permissions(path, perm).await
}

#[inline]
pub async fn symlink_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    get_runtime().fs().symlink_metadata(path).await
}

#[inline]
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    get_runtime().fs().write(path, contents).await
}

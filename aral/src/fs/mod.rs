use crate::current_runtime;
use aral_trait::fs::Fs;
use std::{
    fs::{Metadata, Permissions},
    io::{Result, SeekFrom},
    path::{Path, PathBuf},
};

#[inline]
pub async fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    current_runtime().canonicalize(path).await
}

#[inline]
pub async fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
    current_runtime().copy(from, to).await
}

#[inline]
pub async fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().create_dir(path).await
}

#[inline]
pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().create_dir_all(path).await
}

#[inline]
pub async fn hard_link(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    current_runtime().hard_link(src, dst).await
}

#[inline]
pub async fn metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    current_runtime().metadata(path).await
}

#[inline]
pub async fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    current_runtime().read(path).await
}

#[inline]
pub async fn read_link(path: impl AsRef<Path>) -> Result<PathBuf> {
    current_runtime().read_link(path).await
}

#[inline]
pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    current_runtime().read_to_string(path).await
}

#[inline]
pub async fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().remove_dir(path).await
}

#[inline]
pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().remove_dir_all(path).await
}

#[inline]
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    current_runtime().remove_file(path).await
}

#[inline]
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    current_runtime().rename(from, to).await
}

#[inline]
pub async fn set_permissions(path: impl AsRef<Path>, perm: Permissions) -> Result<()> {
    current_runtime().set_permissions(path, perm).await
}

#[inline]
pub async fn symlink_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    current_runtime().symlink_metadata(path).await
}

#[inline]
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    current_runtime().write(path, contents).await
}

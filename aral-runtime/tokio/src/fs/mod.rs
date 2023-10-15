use aral_trait::{
    fs::{File, Fs, OpenOptions},
    io::{Read, Seek, Write},
};
use std::{
    fs::Metadata,
    future::Future,
    io::Result,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub struct TokioFile(tokio::fs::File);

impl File for TokioFile {
    #[inline]
    fn metadata(&self) -> impl Future<Output = Result<Metadata>> {
        self.0.metadata()
    }

    #[inline]
    fn set_len(&self, size: u64) -> impl Future<Output = Result<()>> {
        self.0.set_len(size)
    }

    #[inline]
    fn set_permissions(&self, perm: std::fs::Permissions) -> impl Future<Output = Result<()>> {
        self.0.set_permissions(perm)
    }

    #[inline]
    fn sync_all(&self) -> impl Future<Output = Result<()>> {
        self.0.sync_all()
    }

    #[inline]
    fn sync_data(&self) -> impl Future<Output = Result<()>> {
        self.0.sync_data()
    }

    #[inline]
    fn try_clone(&self) -> impl Future<Output = Result<impl File>> {
        async move { self.0.try_clone().await.map(TokioFile) }
    }
}

impl Read for TokioFile {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        async move { AsyncReadExt::read(&mut self.0, buf).await }
    }
}

impl Write for TokioFile {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize>> {
        async move { AsyncWriteExt::write(&mut self.0, buf).await }
    }

    #[inline]
    fn flush(&mut self) -> impl Future<Output = Result<()>> {
        async move { AsyncWriteExt::flush(&mut self.0).await }
    }
}

impl Seek for TokioFile {
    #[inline]
    fn seek(&mut self, pos: std::io::SeekFrom) -> impl Future<Output = Result<u64>> {
        async move { AsyncSeekExt::seek(&mut self.0, pos).await }
    }
}

#[derive(Default)]
pub struct TokioOpenOptions(tokio::fs::OpenOptions);

impl OpenOptions for TokioOpenOptions {
    #[inline]
    fn append(&mut self, append: bool) -> &mut Self {
        self.0.append(append);
        self
    }

    #[inline]
    fn create(&mut self, create: bool) -> &mut Self {
        self.0.create(create);
        self
    }

    #[inline]
    fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.0.create_new(create_new);
        self
    }

    fn open(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File + '_>> {
        async move { self.0.open(path).await.map(TokioFile) }
    }

    #[inline]
    fn read(&mut self, read: bool) -> &mut Self {
        self.0.read(read);
        self
    }

    #[inline]
    fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.0.truncate(truncate);
        self
    }

    #[inline]
    fn write(&mut self, write: bool) -> &mut Self {
        self.0.write(write);
        self
    }
}

pub struct TokioFs;

impl Fs for TokioFs {
    fn create_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>> {
        async move { tokio::fs::File::create(path).await.map(TokioFile) }
    }

    fn open_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>> {
        async move { tokio::fs::File::open(path).await.map(TokioFile) }
    }

    fn open_options(&self) -> impl OpenOptions {
        TokioOpenOptions(tokio::fs::OpenOptions::new())
    }

    fn canonicalize(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>> {
        tokio::fs::canonicalize(path)
    }

    fn copy(
        &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    ) -> impl Future<Output = Result<u64>> {
        tokio::fs::copy(from, to)
    }

    fn create_dir(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        tokio::fs::create_dir(path)
    }

    fn create_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        tokio::fs::create_dir_all(path)
    }

    fn hard_link(
        &self, src: impl AsRef<Path>, dst: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>> {
        tokio::fs::hard_link(src, dst)
    }

    fn metadata(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>> {
        tokio::fs::metadata(path)
    }

    fn read(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>>> {
        tokio::fs::read(path)
    }

    fn read_link(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>> {
        tokio::fs::read_link(path)
    }

    fn read_to_string(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<String>> {
        tokio::fs::read_to_string(path)
    }

    fn remove_dir(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        tokio::fs::remove_dir(path)
    }

    fn remove_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        tokio::fs::remove_dir_all(path)
    }

    fn remove_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        tokio::fs::remove_file(path)
    }

    fn rename(
        &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>> {
        tokio::fs::rename(from, to)
    }

    fn set_permissions(
        &self, path: impl AsRef<Path>, perm: std::fs::Permissions,
    ) -> impl Future<Output = Result<()>> {
        tokio::fs::set_permissions(path, perm)
    }

    fn symlink_metadata(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>> {
        tokio::fs::symlink_metadata(path)
    }

    fn write(
        &self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = Result<()>> {
        tokio::fs::write(path, contents)
    }
}

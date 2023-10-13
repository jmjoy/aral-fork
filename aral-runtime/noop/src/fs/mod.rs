use crate::{Noop, Runtime};
use aral_trait::{
    fs::{File, Fs, OpenOptions},
    io::{Read, Seek, Write},
};
use std::{
    future::Future,
    io::Result,
    path::{Path, PathBuf},
};

impl File for Noop {
    #[inline]
    fn metadata(&self) -> impl Future<Output = Result<std::fs::Metadata>> {
        async move {
            no_adapter_specified!();
        }
    }

    #[inline]
    fn set_len(&self, size: u64) -> impl Future<Output = Result<()>> {
        async move {
            no_adapter_specified!();
        }
    }

    #[inline]
    fn set_permissions(&self, perm: std::fs::Permissions) -> impl Future<Output = Result<()>> {
        async move {
            no_adapter_specified!();
        }
    }

    #[inline]
    fn sync_all(&self) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    #[inline]
    fn sync_data(&self) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    #[inline]
    fn try_clone(&self) -> impl Future<Output = Result<impl File>> {
        no_adapter_specified!();
        async move { Ok(Noop) }
    }
}

impl Read for Noop {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        async move {
            no_adapter_specified!();
        }
    }
}

impl Write for Noop {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize>> {
        async move {
            no_adapter_specified!();
        }
    }

    #[inline]
    fn flush(&mut self) -> impl Future<Output = Result<()>> {
        async move {
            no_adapter_specified!();
        }
    }
}

impl Seek for Noop {
    #[inline]
    fn seek(&mut self, pos: std::io::SeekFrom) -> impl Future<Output = Result<u64>> {
        async move {
            no_adapter_specified!();
        }
    }
}

impl OpenOptions for Noop {
    #[inline]
    fn append(&mut self, append: bool) -> &mut Self {
        no_adapter_specified!();
    }

    #[inline]
    fn create(&mut self, create: bool) -> &mut Self {
        no_adapter_specified!();
    }

    #[inline]
    fn create_new(&mut self, create_new: bool) -> &mut Self {
        no_adapter_specified!();
    }

    fn open(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>> {
        no_adapter_specified!();
        async move { Ok(Noop) }
    }

    #[inline]
    fn read(&mut self, read: bool) -> &mut Self {
        no_adapter_specified!();
    }

    #[inline]
    fn truncate(&mut self, truncate: bool) -> &mut Self {
        no_adapter_specified!();
    }

    #[inline]
    fn write(&mut self, write: bool) -> &mut Self {
        no_adapter_specified!();
    }
}

impl Fs for Runtime {
    fn create_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>> {
        no_adapter_specified!();
        async move { Ok(Noop) }
    }

    fn open_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<impl File>> {
        no_adapter_specified!();
        async move { Ok(Noop) }
    }

    fn open_options(&self) -> impl OpenOptions {
        no_adapter_specified!();
        Noop
    }

    fn canonicalize(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn copy(
        &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    ) -> impl Future<Output = Result<u64>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn create_dir(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn create_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn hard_link(
        &self, src: impl AsRef<Path>, dst: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn metadata(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<std::fs::Metadata>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn read(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn read_link(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn read_to_string(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<String>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn remove_dir(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn remove_dir_all(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn remove_file(&self, path: impl AsRef<Path>) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn rename(
        &self, from: impl AsRef<Path>, to: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn set_permissions(
        &self, path: impl AsRef<Path>, perm: std::fs::Permissions,
    ) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn symlink_metadata(
        &self, path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<std::fs::Metadata>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }

    fn write(
        &self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = Result<()>> {
        no_adapter_specified!();
        async move {
            no_adapter_specified!();
        }
    }
}

use super::{ResumableRecorder, SourceKey};
use digest::Digest;
use sha1::Sha1;
use std::{
    fmt::{self, Debug},
    io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write},
    marker::PhantomData,
};

#[cfg(feature = "async")]
use futures::future::BoxFuture;

#[derive(Clone, Copy)]
pub struct DummyResumableRecorder<O = Sha1> {
    _unused: PhantomData<O>,
}

impl<O> DummyResumableRecorder<O> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl<O> Default for DummyResumableRecorder<O> {
    #[inline]
    fn default() -> Self {
        Self {
            _unused: Default::default(),
        }
    }
}

impl<O: Digest + Send + Sync + Unpin> ResumableRecorder for DummyResumableRecorder<O> {
    type HashAlgorithm = O;
    type ReadOnlyMedium = DummyResumableRecorderMedium;
    type AppendOnlyMedium = DummyResumableRecorderMedium;

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    type AsyncReadOnlyMedium = DummyResumableRecorderMedium;

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    type AsyncAppendOnlyMedium = DummyResumableRecorderMedium;

    #[inline]
    fn open_for_read(
        &self,
        _source_key: &SourceKey<Self::HashAlgorithm>,
    ) -> IoResult<Self::ReadOnlyMedium> {
        Err(make_error())
    }

    #[inline]
    fn open_for_append(
        &self,
        _source_key: &SourceKey<Self::HashAlgorithm>,
    ) -> IoResult<Self::AppendOnlyMedium> {
        Err(make_error())
    }

    #[inline]
    fn open_for_create_new(
        &self,
        _source_key: &SourceKey<Self::HashAlgorithm>,
    ) -> IoResult<Self::AppendOnlyMedium> {
        Err(make_error())
    }

    #[inline]
    fn delete(&self, _source_key: &SourceKey<Self::HashAlgorithm>) -> IoResult<()> {
        Err(make_error())
    }

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn open_for_async_read<'a>(
        &'a self,
        _source_key: &'a SourceKey<Self::HashAlgorithm>,
    ) -> BoxFuture<'a, IoResult<Self::AsyncReadOnlyMedium>> {
        Box::pin(async move { Err(make_error()) })
    }

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn open_for_async_append<'a>(
        &'a self,
        _source_key: &'a SourceKey<Self::HashAlgorithm>,
    ) -> BoxFuture<'a, IoResult<Self::AsyncAppendOnlyMedium>> {
        Box::pin(async move { Err(make_error()) })
    }

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn open_for_async_create_new<'a>(
        &'a self,
        _source_key: &'a SourceKey<Self::HashAlgorithm>,
    ) -> BoxFuture<'a, IoResult<Self::AsyncAppendOnlyMedium>> {
        Box::pin(async move { Err(make_error()) })
    }

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_delete<'a>(
        &'a self,
        _source_key: &'a SourceKey<Self::HashAlgorithm>,
    ) -> BoxFuture<'a, IoResult<()>> {
        Box::pin(async move { Err(make_error()) })
    }
}

impl<O> Debug for DummyResumableRecorder<O> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DummyResumableRecorder").finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DummyResumableRecorderMedium;

impl Read for DummyResumableRecorderMedium {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> IoResult<usize> {
        Err(make_error())
    }
}

impl Write for DummyResumableRecorderMedium {
    #[inline]
    fn write(&mut self, _buf: &[u8]) -> IoResult<usize> {
        Err(make_error())
    }

    #[inline]
    fn flush(&mut self) -> IoResult<()> {
        Err(make_error())
    }
}

#[cfg(feature = "async")]
use {
    futures::{AsyncRead, AsyncWrite},
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

#[cfg(feature = "async")]
impl AsyncRead for DummyResumableRecorderMedium {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut [u8],
    ) -> Poll<IoResult<usize>> {
        Poll::Ready(Err(make_error()))
    }
}

#[cfg(feature = "async")]
impl AsyncWrite for DummyResumableRecorderMedium {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<IoResult<usize>> {
        Poll::Ready(Err(make_error()))
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<IoResult<()>> {
        Poll::Ready(Err(make_error()))
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<IoResult<()>> {
        Poll::Ready(Err(make_error()))
    }
}

fn make_error() -> IoError {
    IoError::new(IoErrorKind::Unsupported, "Unimplemented")
}
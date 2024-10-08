use core::pin::Pin;
use core::future::Future;
use core::task::{Context, Poll};

use crate::{VfsNodeOps, VfsResult};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct FsyncFuture<'a, T: Unpin + ?Sized> {
    pub(crate) vnode: &'a T,
}

impl<T: VfsNodeOps + Unpin + ?Sized> Future for FsyncFuture<'_, T> {
    type Output = VfsResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { vnode } = self.get_mut();
        Pin::new(*vnode).fsync(cx)
    }
}
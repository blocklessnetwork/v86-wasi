use std::{io::{self, Read, Write}, task::Poll};

use futures::{AsyncRead, AsyncWrite};

use crate::{Device, Interest, Tap};

use super::reactor::{try_invoke, self};

impl AsyncRead for Tap {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        try_invoke(self.token(), cx.waker().clone(), Interest::READABLE, || {
            self.read(buf)
        })
    }
}

impl AsyncWrite for Tap {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        try_invoke(self.token(), cx.waker().clone(), Interest::WRITABLE,||{
            self.write(buf)
        })
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>, 
        _cx: &mut std::task::Context<'_>
    ) -> Poll<io::Result<()>> {
        Poll::Ready(self.flush())
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>, 
        _cx: &mut std::task::Context<'_>
    ) -> Poll<io::Result<()>> {
        reactor::remove_token(self.token());
        Poll::Ready(Ok(()))
    }
}




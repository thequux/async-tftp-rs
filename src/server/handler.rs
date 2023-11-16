use async_trait::async_trait;
use futures_lite::{AsyncRead, AsyncWrite};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use crate::packet;
use crate::packet::Error;

/// Trait for implementing advance handlers.
#[crate::async_trait]
pub trait Handler: Send{
    type Reader: AsyncRead + Unpin + Send + 'static;
    type Writer: AsyncWrite + Unpin + Send + 'static;

    /// Open `Reader` to serve a read request.
    async fn read_req_open(
        &mut self,
        client: &SocketAddr,
        path: &Path,
    ) -> Result<(Self::Reader, Option<u64>), packet::Error>;

    /// Open `Writer` to serve a write request.
    async fn write_req_open(
        &mut self,
        client: &SocketAddr,
        path: &Path,
        size: Option<u64>,
    ) -> Result<Self::Writer, packet::Error>;
}

#[async_trait]
impl<H: Handler> Handler for Arc<async_lock::Mutex<H>> {
    type Reader = H::Reader;
    type Writer = H::Writer;

    async fn read_req_open(&mut self, client: &SocketAddr, path: &Path) -> Result<(Self::Reader, Option<u64>), Error> {
        self.lock_arc().await.read_req_open(client, path).await
    }

    async fn write_req_open(&mut self, client: &SocketAddr, path: &Path, size: Option<u64>) -> Result<Self::Writer, Error> {
        self.lock_arc().await.write_req_open(client, path, size).await
    }
}

#[async_trait]
impl<H: Handler + Sync> Handler for Arc<H> {
    type Reader = H::Reader;
    type Writer = H::Writer;

    async fn write_req_open(&mut self, client: &SocketAddr, path: &Path, size: Option<u64>) -> Result<Self::Writer, Error> {
        self.write_req_open(client, path, size).await
    }

    async fn read_req_open(&mut self, client: &SocketAddr, path: &Path) -> Result<(Self::Reader, Option<u64>), Error> {
        self.read_req_open(client, path).await
    }
}
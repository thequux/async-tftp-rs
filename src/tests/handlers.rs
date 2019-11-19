use futures::channel::oneshot;
use futures::io::Sink;
use std::net::SocketAddr;
use std::path::Path;

use super::random_file::RandomFile;
use crate::packet;
use crate::server::Handler;

pub struct RandomHandler {
    md5_tx: Option<oneshot::Sender<md5::Digest>>,
    file_size: usize,
}

impl RandomHandler {
    pub fn new(file_size: usize, md5_tx: oneshot::Sender<md5::Digest>) -> Self {
        RandomHandler {
            md5_tx: Some(md5_tx),
            file_size,
        }
    }
}

#[crate::async_trait]
impl Handler for RandomHandler {
    type Reader = RandomFile;
    #[cfg(feature = "unstable")]
    type Writer = Sink;

    async fn read_req_open(
        &mut self,
        _client: &SocketAddr,
        _path: &Path,
    ) -> Result<(Self::Reader, Option<u64>), packet::Error> {
        let md5_tx = self.md5_tx.take().expect("md5_tx already consumed");
        Ok((RandomFile::new(self.file_size, md5_tx), None))
    }

    #[cfg(feature = "unstable")]
    async fn write_open(
        &mut self,
        _client: &SocketAddr,
        _path: &Path,
        _size: Option<u64>,
    ) -> Result<Self::Writer, packet::Error> {
        Err(packet::Error::IllegalOperation)
    }
}
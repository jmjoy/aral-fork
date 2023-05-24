use crate::{
    io::{Read, Write},
    net::ToSocketAddrs,
};
use std::{io::Result, net::SocketAddr};

pub struct TcpStream(tokio::net::TcpStream);

impl TcpStream {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        tokio::net::TcpStream::connect(&*addrs).await.map(TcpStream)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }

    #[inline]
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr()
    }
}

impl Read for TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        tokio::io::AsyncReadExt::read(&mut self.0, buf).await
    }
}

impl Write for TcpStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        tokio::io::AsyncWriteExt::write(&mut self.0, buf).await
    }

    async fn flush(&mut self) -> Result<()> {
        tokio::io::AsyncWriteExt::flush(&mut self.0).await
    }
}

pub struct TcpListener(tokio::net::TcpListener);

impl TcpListener {
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (TcpStream(stream), addr))
    }

    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        tokio::net::TcpListener::bind(&*addrs)
            .await
            .map(TcpListener)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }
}

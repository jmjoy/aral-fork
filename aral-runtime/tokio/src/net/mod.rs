use aral_trait::{
    io::{Read, Write},
    net::{Net, TcpListener, TcpStream, ToSocketAddrSolutions, ToSocketAddrs, UdpSocket},
};
use std::{future::Future, io::Result, net::SocketAddr};
use tokio::net::lookup_host;

struct TokioToSocketAddrSolutions;

impl ToSocketAddrSolutions for TokioToSocketAddrSolutions {
    fn str_u16(
        &self, addr: &(&str, u16),
    ) -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
        }
    }

    fn ip_addr_u16(
        &self, addr: &(std::net::IpAddr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().next().into_iter())
        }
    }

    fn string_u16(
        &self, addr: &(String, u16),
    ) -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
        }
    }

    fn ipv4_addr_u16(
        &self, addr: &(std::net::Ipv4Addr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().next().into_iter())
        }
    }

    fn ipv6_addr_u16(
        &self, addr: &(std::net::Ipv6Addr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().next().into_iter())
        }
    }

    fn socket_addr(
        &self, addr: &SocketAddr,
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().next().into_iter())
        }
    }

    fn str(&self, addr: &str) -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
        }
    }

    fn string(&self, addr: &String)
        -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>> {
            async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
            }
    }

    fn socket_addr_v4(
        &self, addr: &std::net::SocketAddrV4,
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().next().into_iter())
        }
    }

    fn socket_addr_v6(
        &self, addr: &std::net::SocketAddrV6,
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>> {
        async move {
        lookup_host(addr)
            .await
            .map(|it| it.into_iter().next().into_iter())
        }
    }
}

pub struct TokioTcpStream(tokio::net::TcpStream);

impl TcpStream for TokioTcpStream {
    #[inline]
    fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }

    #[inline]
    fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr()
    }

    #[inline]
    fn nodelay(&self) -> Result<bool> {
        self.0.nodelay()
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        self.0.peek(buf)
    }

    #[inline]
    fn set_nodelay(&self, nodelay: bool) -> Result<()> {
        self.0.set_nodelay(nodelay)
    }

    #[inline]
    fn set_ttl(&self, ttl: u32) -> Result<()> {
        self.0.set_ttl(ttl)
    }

    #[inline]
    fn ttl(&self) -> Result<u32> {
        self.0.ttl()
    }
}

impl Read for TokioTcpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        async move { tokio::io::AsyncReadExt::read(&mut self.0, buf).await }
    }
}

impl Write for TokioTcpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize>> {
        async move { tokio::io::AsyncWriteExt::write(&mut self.0, buf).await }
    }

    #[inline]
    fn flush(&mut self) -> impl Future<Output = Result<()>> {
        async move { tokio::io::AsyncWriteExt::flush(&mut self.0).await }
    }
}

pub struct TokioTcpListener(tokio::net::TcpListener);

impl TcpListener for TokioTcpListener {
    fn accept(&self) -> impl Future<Output = Result<(impl TcpStream, SocketAddr)>> {
        async move {
            self.0
                .accept()
                .await
                .map(|(stream, addr)| (TokioTcpStream(stream), addr))
        }
    }

    #[inline]
    fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }
}

pub struct TokioUdpSocket(tokio::net::UdpSocket);

impl UdpSocket for TokioUdpSocket {
    #[inline]
    fn broadcast(&self) -> Result<bool> {
        self.0.broadcast()
    }

    #[inline]
    fn connect(&self, addr: impl ToSocketAddrs) -> impl Future<Output = Result<()>> {
        async move {
            let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
            self.0.connect(&*addrs).await
        }
    }

    #[inline]
    fn join_multicast_v4(
        &self, multiaddr: &std::net::Ipv4Addr, interface: &std::net::Ipv4Addr,
    ) -> Result<()> {
        self.0.join_multicast_v4(*multiaddr, *interface)
    }

    #[inline]
    fn join_multicast_v6(&self, multiaddr: &std::net::Ipv6Addr, interface: u32) -> Result<()> {
        self.0.join_multicast_v6(multiaddr, interface)
    }

    #[inline]
    fn leave_multicast_v4(
        &self, multiaddr: &std::net::Ipv4Addr, interface: &std::net::Ipv4Addr,
    ) -> Result<()> {
        self.0.leave_multicast_v4(*multiaddr, *interface)
    }

    #[inline]
    fn leave_multicast_v6(&self, multiaddr: &std::net::Ipv6Addr, interface: u32) -> Result<()> {
        self.0.leave_multicast_v6(multiaddr, interface)
    }

    #[inline]
    fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }

    #[inline]
    fn multicast_loop_v4(&self) -> Result<bool> {
        self.0.multicast_loop_v4()
    }

    #[inline]
    fn multicast_loop_v6(&self) -> Result<bool> {
        self.0.multicast_loop_v6()
    }

    #[inline]
    fn multicast_ttl_v4(&self) -> Result<u32> {
        self.0.multicast_ttl_v4()
    }

    #[inline]
    fn peek_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>> {
        self.0.peek_from(buf)
    }

    #[inline]
    fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr()
    }

    #[inline]
    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        self.0.recv(buf)
    }

    #[inline]
    fn recv_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>> {
        self.0.recv_from(buf)
    }

    #[inline]
    fn send(&self, buf: &[u8]) -> impl Future<Output = Result<usize>> {
        self.0.send(buf)
    }

    #[inline]
    fn send_to(
        &self, buf: &[u8], target: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<usize>> {
        async move {
            let target = target.to_socket_addrs().await?.collect::<Vec<_>>();
            self.0.send_to(buf, &*target).await
        }
    }

    #[inline]
    fn set_broadcast(&self, on: bool) -> Result<()> {
        self.0.set_broadcast(on)
    }

    #[inline]
    fn set_multicast_loop_v4(&self, on: bool) -> Result<()> {
        self.0.set_multicast_loop_v4(on)
    }

    #[inline]
    fn set_multicast_loop_v6(&self, on: bool) -> Result<()> {
        self.0.set_multicast_loop_v6(on)
    }

    #[inline]
    fn set_multicast_ttl_v4(&self, ttl: u32) -> Result<()> {
        self.0.set_multicast_ttl_v4(ttl)
    }

    #[inline]
    fn set_ttl(&self, ttl: u32) -> Result<()> {
        self.0.set_ttl(ttl)
    }

    #[inline]
    fn ttl(&self) -> Result<u32> {
        self.0.ttl()
    }
}

pub struct TokioNet;

impl Net for TokioNet {
    #[inline]
    fn to_socket_addr_solutions(&self) -> &'static impl ToSocketAddrSolutions {
        &TokioToSocketAddrSolutions
    }

    fn connect_tcp_stream(
        &self, addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl TcpStream>> {
        async move {
            let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
            tokio::net::TcpStream::connect(&*addrs)
                .await
                .map(TokioTcpStream)
        }
    }

    fn bind_tcp_listener(
        &self, addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl TcpListener>> {
        async move {
            let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
            tokio::net::TcpListener::bind(&*addrs)
                .await
                .map(TokioTcpListener)
        }
    }

    fn bind_udp_socket(
        &self, addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl UdpSocket>> {
        async move {
            let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
            tokio::net::UdpSocket::bind(&*addrs)
                .await
                .map(TokioUdpSocket)
        }
    }
}

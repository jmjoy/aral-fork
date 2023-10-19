use crate::{Noop, NoopRuntime};
use aral_trait::net::{Net, TcpListener, TcpStream, ToSocketAddrs, UdpSocket, ToSocketAddrSolutions};
use std::{future::Future, io::Result};

impl ToSocketAddrSolutions for Noop {
    fn str_u16(
        &self, addr: &(&str, u16),
    ) -> impl Future<Output = Result<std::vec::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn ip_addr_u16(
        &self, addr: &(std::net::IpAddr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn string_u16(
        &self, addr: &(String, u16),
    ) -> impl Future<Output = Result<std::vec::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn ipv4_addr_u16(
        &self, addr: &(std::net::Ipv4Addr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn ipv6_addr_u16(
        &self, addr: &(std::net::Ipv6Addr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn socket_addr(
        &self, addr: &std::net::SocketAddr,
    ) -> impl Future<Output = Result<std::option::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn str(&self, addr: &str) -> impl Future<Output = Result<std::vec::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn string(&self, addr: &String)
        -> impl Future<Output = Result<std::vec::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn socket_addr_v4(
        &self, addr: &std::net::SocketAddrV4,
    ) -> impl Future<Output = Result<std::option::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }

    fn socket_addr_v6(
        &self, addr: &std::net::SocketAddrV6,
    ) -> impl Future<Output = Result<std::option::IntoIter<std::net::SocketAddr>>> {
        no_runtime_specified!();
        async move {
        no_runtime_specified!();
        }
    }
}

impl TcpStream for Noop {
    #[inline]
    fn local_addr(&self) -> Result<std::net::SocketAddr> {
        no_runtime_specified!();
    }

    #[inline]
    fn peer_addr(&self) -> Result<std::net::SocketAddr> {
        no_runtime_specified!();
    }

    #[inline]
    fn nodelay(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    #[inline]
    fn peek(&self, _buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn set_nodelay(&self, _nodelay: bool) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn set_ttl(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn ttl(&self) -> Result<u32> {
        no_runtime_specified!();
    }
}

impl TcpListener for Noop {
    #[inline]
    fn accept(&self) -> impl Future<Output = Result<(impl TcpStream, std::net::SocketAddr)>> {
        async move { Ok((Noop, "0.0.0.0:0".parse().unwrap())) }
    }

    #[inline]
    fn local_addr(&self) -> Result<std::net::SocketAddr> {
        no_runtime_specified!();
    }
}

impl UdpSocket for Noop {
    #[inline]
    fn broadcast(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    #[inline]
    fn connect(&self, _addr: impl ToSocketAddrs) -> impl Future<Output = Result<()>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn join_multicast_v4(
        &self, _multiaddr: &std::net::Ipv4Addr, _interface: &std::net::Ipv4Addr,
    ) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn join_multicast_v6(&self, _multiaddr: &std::net::Ipv6Addr, _interface: u32) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn leave_multicast_v4(
        &self, _multiaddr: &std::net::Ipv4Addr, _interface: &std::net::Ipv4Addr,
    ) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn leave_multicast_v6(&self, _multiaddr: &std::net::Ipv6Addr, _interface: u32) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn local_addr(&self) -> Result<std::net::SocketAddr> {
        no_runtime_specified!();
    }

    #[inline]
    fn multicast_loop_v4(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    #[inline]
    fn multicast_loop_v6(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    #[inline]
    fn multicast_ttl_v4(&self) -> Result<u32> {
        no_runtime_specified!();
    }

    #[inline]
    fn peek_from(
        &self, _buf: &mut [u8],
    ) -> impl Future<Output = Result<(usize, std::net::SocketAddr)>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn peer_addr(&self) -> Result<std::net::SocketAddr> {
        no_runtime_specified!();
    }

    #[inline]
    fn recv(&self, _buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn recv_from(
        &self, _buf: &mut [u8],
    ) -> impl Future<Output = Result<(usize, std::net::SocketAddr)>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn send(&self, _buf: &[u8]) -> impl Future<Output = Result<usize>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn send_to(
        &self, _buf: &[u8], _target: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<usize>> {
        async move {
            no_runtime_specified!();
        }
    }

    #[inline]
    fn set_broadcast(&self, _on: bool) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn set_multicast_loop_v4(&self, _on: bool) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn set_multicast_loop_v6(&self, _on: bool) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn set_multicast_ttl_v4(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn set_ttl(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    fn ttl(&self) -> Result<u32> {
        no_runtime_specified!();
    }
}

impl Net for NoopRuntime {
    #[inline]
    fn to_socket_addr_solutions(&self) -> &'static impl ToSocketAddrSolutions {
        &Noop
    }
    
    #[inline]
    fn connect_tcp_stream(
        &self, _addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl TcpStream>> {
        no_runtime_specified!();
        async move {
            no_runtime_specified!();
            Ok(Noop)
        }
    }

    #[inline]
    fn bind_tcp_listener(
        &self, _addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl TcpListener>> {
        no_runtime_specified!();
        async move {
            no_runtime_specified!();
            Ok(Noop)
        }
    }

    #[inline]
    fn bind_udp_socket(
        &self, _addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl UdpSocket>> {
        no_runtime_specified!();
        async move {
            no_runtime_specified!();
            Ok(Noop)
        }
    }
}

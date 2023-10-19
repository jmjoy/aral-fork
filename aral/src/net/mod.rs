use crate::{current_runtime, Runtime};
use crate::io::{Read, Write};
use std::{
    future::Future,
    io::Result,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    fn to_socket_addrs(&self) -> impl Future<Output = Result<Self::Iter>>;
}

pub trait ToSocketAddrSolutions {
    fn str_u16(
        &self, addr: &(&str, u16),
    ) -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>>;

    fn ip_addr_u16(
        &self, addr: &(IpAddr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>>;

    fn string_u16(
        &self, addr: &(String, u16),
    ) -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>>;

    fn ipv4_addr_u16(
        &self, addr: &(Ipv4Addr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>>;

    fn ipv6_addr_u16(
        &self, addr: &(Ipv6Addr, u16),
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>>;

    fn socket_addr(
        &self, addr: &SocketAddr,
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>>;

    fn str(&self, addr: &str) -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>>;

    fn string(&self, addr: &String)
        -> impl Future<Output = Result<std::vec::IntoIter<SocketAddr>>>;

    fn socket_addr_v4(
        &self, addr: &SocketAddrV4,
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>>;

    fn socket_addr_v6(
        &self, addr: &SocketAddrV6,
    ) -> impl Future<Output = Result<std::option::IntoIter<SocketAddr>>>;

    fn socket_addrs<'a>(
        &self, addr: &&'a [SocketAddr],
    ) -> impl Future<Output = Result<std::iter::Cloned<std::slice::Iter<'a, SocketAddr>>>> {
        async move {
            Ok(addr.iter().cloned())
        }
    }

    fn to_socket_addrs<T: ToSocketAddrs + ?Sized>(
        &self, addr: &&T,
    ) -> impl Future<Output = Result<<T as ToSocketAddrs>::Iter>> {
        (**addr).to_socket_addrs()
    }
}

pub trait TcpStream: Read + Write {
    fn local_addr(&self) -> Result<SocketAddr>;

    fn peer_addr(&self) -> Result<SocketAddr>;

    fn nodelay(&self) -> Result<bool>;

    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;

    fn set_nodelay(&self, nodelay: bool) -> Result<()>;

    fn set_ttl(&self, ttl: u32) -> Result<()>;

    fn ttl(&self) -> Result<u32>;
}

pub trait TcpListener {
    fn accept(&self) -> impl Future<Output = Result<(impl TcpStream, SocketAddr)>>;

    fn local_addr(&self) -> Result<SocketAddr>;
}

pub trait UdpSocket {
    fn broadcast(&self) -> Result<bool>;

    fn connect(&self, addr: impl ToSocketAddrs) -> impl Future<Output = Result<()>>;

    fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<()>;

    fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()>;

    fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<()>;

    fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()>;

    fn local_addr(&self) -> Result<SocketAddr>;

    fn multicast_loop_v4(&self) -> Result<bool>;

    fn multicast_loop_v6(&self) -> Result<bool>;

    fn multicast_ttl_v4(&self) -> Result<u32>;

    fn peek_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>>;

    fn peer_addr(&self) -> Result<SocketAddr>;

    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;

    fn recv_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>>;

    fn send(&self, buf: &[u8]) -> impl Future<Output = Result<usize>>;

    fn send_to(
        &self, buf: &[u8], target: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<usize>>;

    fn set_broadcast(&self, on: bool) -> Result<()>;

    fn set_multicast_loop_v4(&self, on: bool) -> Result<()>;

    fn set_multicast_loop_v6(&self, on: bool) -> Result<()>;

    fn set_multicast_ttl_v4(&self, ttl: u32) -> Result<()>;

    fn set_ttl(&self, ttl: u32) -> Result<()>;

    fn ttl(&self) -> Result<u32>;
}

pub trait Net {
    fn to_socket_addr_solutions(&self) -> &'static impl ToSocketAddrSolutions;

    fn connect_tcp_stream(
        &self, addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl TcpStream>>;

    fn bind_tcp_listener(
        &self, addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl TcpListener>>;

    fn bind_udp_socket(
        &self, addr: impl ToSocketAddrs,
    ) -> impl Future<Output = Result<impl UdpSocket>>;
}



pub async fn connect_tcp_stream(addr: impl ToSocketAddrs) -> Result<impl TcpStream> {
    current_runtime().net().connect_tcp_stream(addr).await
}

pub async fn bind_tcp_listener(addr: impl ToSocketAddrs) -> Result<impl TcpListener> {
    current_runtime().net().bind_tcp_listener(addr).await
}

pub async fn bind_udp_socket(addr: impl ToSocketAddrs) -> Result<impl UdpSocket> {
    current_runtime().net().bind_udp_socket(addr).await
}

//! Tiny networking helpers shared by the DLNA server and its Tauri commands.

use std::net::{Ipv4Addr, SocketAddr};

/// Detect the LAN IPv4 address by opening a UDP socket to a routable target.
/// No packet is actually sent (UDP has no handshake) but the OS picks the
/// outgoing interface for us. Falls back to loopback if detection fails.
///
/// Used both for SSDP `LOCATION` headers and for building the absolute
/// media URLs embedded in DIDL responses.
pub fn detect_local_ipv4() -> Ipv4Addr {
    let socket = match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return Ipv4Addr::LOCALHOST,
    };
    if socket.connect("8.8.8.8:80").is_err() {
        return Ipv4Addr::LOCALHOST;
    }
    match socket.local_addr() {
        Ok(SocketAddr::V4(v4)) => *v4.ip(),
        _ => Ipv4Addr::LOCALHOST,
    }
}

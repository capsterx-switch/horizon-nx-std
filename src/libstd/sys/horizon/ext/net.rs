#![stable(feature = "unix_socket", since = "1.10.0")]

//! Unix-specific networking functionality

#[cfg(unix)]
use libc;

// FIXME(#43348): Make libc adapt #[doc(cfg(...))] so we don't need these fake definitions here?
#[cfg(not(unix))]
mod libc {
    pub use libc::c_int;
    pub type socklen_t = u32;
    pub struct sockaddr;
}

use ascii;
use ffi::OsStr;
use fmt;
use io::{self, Initializer};
use mem;
use net::{self, Shutdown};
use os::unix::ffi::OsStrExt;
use os::unix::io::{RawFd, AsRawFd, FromRawFd, IntoRawFd};
use path::Path;
use time::Duration;
use sys::{self, cvt};
use sys::net::Socket;
use sys_common::{self, AsInner, FromInner, IntoInner};

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawFd for net::TcpStream {
    fn as_raw_fd(&self) -> RawFd { *self.as_inner().socket().as_inner() }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawFd for net::TcpListener {
    fn as_raw_fd(&self) -> RawFd { *self.as_inner().socket().as_inner() }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawFd for net::UdpSocket {
    fn as_raw_fd(&self) -> RawFd { *self.as_inner().socket().as_inner() }
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawFd for net::TcpStream {
    unsafe fn from_raw_fd(fd: RawFd) -> net::TcpStream {
        let socket = sys::net::Socket::from_inner(fd);
        net::TcpStream::from_inner(sys_common::net::TcpStream::from_inner(socket))
    }
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawFd for net::TcpListener {
    unsafe fn from_raw_fd(fd: RawFd) -> net::TcpListener {
        let socket = sys::net::Socket::from_inner(fd);
        net::TcpListener::from_inner(sys_common::net::TcpListener::from_inner(socket))
    }
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawFd for net::UdpSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> net::UdpSocket {
        let socket = sys::net::Socket::from_inner(fd);
        net::UdpSocket::from_inner(sys_common::net::UdpSocket::from_inner(socket))
    }
}

#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawFd for net::TcpStream {
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_socket().into_inner()
    }
}
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawFd for net::TcpListener {
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_socket().into_inner()
    }
}
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawFd for net::UdpSocket {
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_socket().into_inner()
    }
}

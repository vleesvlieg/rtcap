use crate::raw_socket::{
    error::{self, Error},
    Mode,
};
use nix::{
    ifaddrs::getifaddrs,
    sys::socket::{bind, socket, AddressFamily, SockType},
};
use snafu::{OptionExt, ResultExt};
use std::os::unix::io::RawFd;

type Result<T> = std::result::Result<T, Error>;

pub struct RawSocket(RawFd);

impl RawSocket {
    pub fn new_blocking() -> Result<Self> {
        Self::new(Mode::Blocking)
    }

    pub fn new_non_blocking() -> Result<Self> {
        Self::new(Mode::NonBlocking)
    }

    pub fn new(mode: Mode) -> Result<Self> {
        let raw_fd = socket(AddressFamily::Packet, SockType::Raw, mode.into(), None)
            .context(error::Create {})?;
        Ok(Self(raw_fd))
    }

    pub fn bind(&self, name: &str) -> Result<()> {
        let address = getifaddrs()
            .context(error::InterfaceList {})?
            .filter(|interface| interface.interface_name == name)
            .filter(|interface| interface.address.is_some())
            .map(|interface| interface.address.unwrap())
            .find(|address| address.family() == AddressFamily::Packet)
            .context(error::InterfaceName { name })?;
        bind(self.0, &address).context(error::Bind { name })
    }
}

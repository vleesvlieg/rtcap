use nix::sys::socket::SockFlag;

pub enum Mode {
    Blocking,
    NonBlocking,
}

impl Into<SockFlag> for Mode {
    fn into(self) -> SockFlag {
        match self {
            Self::Blocking => SockFlag::empty(),
            Self::NonBlocking => SockFlag::SOCK_NONBLOCK,
        }
    }
}

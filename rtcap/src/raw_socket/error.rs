use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Unable to create raw socket: {}", source))]
    Create { source: nix::Error },

    #[snafu(display("Unable to list network interfaces: {}", source))]
    InterfaceList { source: nix::Error },

    #[snafu(display("Unable to get address for interface {}", name))]
    InterfaceName { name: String },

    #[snafu(display("Unable to bind to interface {}: {}", name, source))]
    Bind { source: nix::Error, name: String },
}

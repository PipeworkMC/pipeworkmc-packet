//! Serverbound packets.


pub mod handshake;
pub mod status;
pub mod login;
pub mod config;
pub mod play;


/// Serverbound packets.
#[derive(Debug)]
pub enum C2SPackets {
    /// Serverbound handshaking packets.
    Handshake(handshake::C2SHandshakePackets),
    /// Serverbound status packets.
    Status(status::C2SStatusPackets),
    /// Serverbound login packets.
    Login(login::C2SLoginPackets),
    /// Serverbound configuration packets.
    Config(config::C2SConfigPackets),
    /// Serverbound play packets.
    Play(play::C2SPlayPackets)
}

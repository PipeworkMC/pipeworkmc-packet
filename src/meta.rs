//! Metadata for packets.


trait Sealed { }


/// Metadata for a packet.
pub trait Packet {
    /// The networked packet ID.
    const PREFIX : u8;
    /// The network direction of the packet.
    type Bound : TBound;
    /// The game state of the packet.
    type State : TState;
    /// Whether this packet will disconnect the peer.
    const KICK : bool = false;
}

/// The network direction of a packet.
#[expect(private_bounds)]
pub trait TBound : Sealed {
    /// As enum.
    const BOUND : PacketBound;
}
/// Packet bounds as types.
#[expect(non_snake_case)]
pub mod Bound {
    /// Serverbound
    pub struct C2S;
    impl super::Sealed for C2S { }
    impl super::TBound for C2S {
        const BOUND : super::PacketBound = super::PacketBound::C2S;
    }
    /// Clientbound
    pub struct S2C;
    impl super::Sealed for S2C { }
    impl super::TBound for S2C {
        const BOUND : super::PacketBound = super::PacketBound::S2C;
    }
}
/// The network direction of a packet.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PacketBound {
    /// Serverbound
    C2S,
    /// Clientbound
    S2C
}

/// The game state of a packet.
#[expect(private_bounds)]
pub trait TState : Sealed {
    /// As enum.
    const STATE : PacketState;
}
/// Game states as types.
#[expect(non_snake_case)]
pub mod State {
    /// Handshaking
    pub struct Handshake;
    impl super::Sealed for Handshake { }
    impl super::TState for Handshake {
        const STATE : super::PacketState = super::PacketState::Handshake;
    }
    /// Status
    pub struct Status;
    impl super::Sealed for Status { }
    impl super::TState for Status {
        const STATE : super::PacketState = super::PacketState::Status;
    }
    /// Login
    pub struct Login;
    impl super::Sealed for Login { }
    impl super::TState for Login {
        const STATE : super::PacketState = super::PacketState::Login;
    }
    /// Configuration
    pub struct Config;
    impl super::Sealed for Config { }
    impl super::TState for Config {
        const STATE : super::PacketState = super::PacketState::Config;
    }
    /// Play
    pub struct Play;
    impl super::Sealed for Play { }
    impl super::TState for Play {
        const STATE : super::PacketState = super::PacketState::Play;
    }
}
/// The game state of a packet.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PacketState {
    /// Handshaking
    Handshake,
    /// Status
    Status,
    /// Login
    Login,
    /// Configuration
    Config,
    /// Play
    Play
}

/// Metadata for a packet.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PacketMeta {
    /// The networked packet ID.
    pub prefix : u8,
    /// The network direction of the packet.
    pub bound  : PacketBound,
    /// The game state of the packet.
    pub state  : PacketState,
    /// Whether this packet will disconnect the peer.
    pub kick   : bool
}
impl PacketMeta {
    /// Get the structured metadata of a packet by type.
    #[inline]
    pub const fn of<P>() -> Self
    where
        P : Packet
    { Self {
        prefix : P::PREFIX,
        bound  : P::Bound::BOUND,
        state  : P::State::STATE,
        kick   : P::KICK
    } }
}

//! Serverbound packets.


use pipeworkmc_codec::{
    decode::{
        PrefixedPacketDecode,
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::PacketMeta
};
use core::fmt::{ self, Display, Formatter };


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


macro packet_group(
    $state:tt $group:ident,
    $err:ident,
    { $( $name:tt $variant:ident => $ty:ty ),* $(,)? }
) {

    #[doc = concat!("Serverbound ", $state, "packets.")]
    #[derive(Debug)]
    pub enum $group { $(
        #[doc = $name]
        $variant( $ty ),
    )* }

    impl PrefixedPacketDecode for $group {
        type Error = $err;

        fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
        where
             I : ExactSizeIterator<Item = u8>
        {
            Ok(match (iter.read().map_err($err::Prefix)?) {
                $( <$ty as PacketMeta>::PREFIX => Self::$variant(<$ty as PacketDecode>::decode(iter).map_err($err::$variant)?) , )*
                v => { return Err($err::UnknownPrefix(v)); }
            })
        }
    }

    #[doc = concat!("Returned by packet decoders when a `", stringify!($group), "` was not decoded successfully.")]
    #[derive(Debug)]
    pub enum $err {
    /// There were not enough bytes to decode a packet ID.
        Prefix(IncompleteDecodeError),
        $(
            #[doc = concat!("A `", stringify!($ty), "` failed to decode.")]
            $variant(<$ty as PacketDecode>::Error),
        )*
        /// An unrecognised packet ID was found.
        UnknownPrefix(u8)
    }

    impl Display for $err {
        fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
            Self::Prefix(err) => err.fmt(f),
            $(
                Self::$variant(err) => write!(f, "{} {err}", $name),
            )*
            Self::UnknownPrefix(b) => write!(f, "unknown prefix `0x{b:0>2X}`")
        } }
    }

}

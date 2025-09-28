//! Serverbound status ping packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


/// Requests a response from the server.
///
/// This is used to calculate latency.
#[derive(Debug)]
pub struct C2SStatusPingPacket {
    /// The timestamp when the client sent the request.
    ///
    /// The server should send this value back.
    pub timestamp : u64
}

impl PacketMeta for C2SStatusPingPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

impl PacketDecode for C2SStatusPingPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self { timestamp : <_>::decode(iter)? }) }
}

//! Serverbound configuration keep alive packet.


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


/// Lets the server know that the connection is still alive.
#[derive(Debug)]
pub struct C2SConfigKeepAlivePacket {
    /// ID of the keepalive. The server previously sent the value to use.
    pub id : u64
}

impl PacketMeta for C2SConfigKeepAlivePacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x04; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigKeepAlivePacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        id : <_>::decode(iter)?
    }) }
}

//! Serverbound play keep alive packet.


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
pub struct C2SPlayKeepAlivePacket {
    /// Transaction ID of the keepalive. The server previously sent the value to use.
    pub transaction : u64
}

impl PacketMeta for C2SPlayKeepAlivePacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("keep_alive");
}

impl PacketDecode for C2SPlayKeepAlivePacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        transaction : <_>::decode(iter)?
    }) }
}

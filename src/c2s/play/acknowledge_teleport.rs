//! Serverbound play acknowledge teleport packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    },
    varint::{
        VarInt,
        VarIntDecodeError
    }
};


/// Lets the server know that a previously received teleport has been received.
#[derive(Debug)]
pub struct C2SPlayAcknowledgeTeleportPacket {
    /// ID of the teleport. The server previously sent the value to use.
    pub id : u32
}

impl PacketMeta for C2SPlayAcknowledgeTeleportPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("accept_teleportation");
}

impl PacketDecode for C2SPlayAcknowledgeTeleportPacket {
    type Error = VarIntDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        id : <VarInt<u32>>::decode(iter)?.0
    }) }
}

//! Clientbound status pong packet.


use crate::s2c::{
    S2CPackets,
    status::S2CStatusPackets
};
use pipeworkmc_codec::{
    encode::{
        PacketEncode,
        EncodeBuf
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


/// Responds to a ping sent by the client.
#[derive(Debug)]
pub struct S2CStatusPongPacket {
    /// The timestamp sent in the original request.
    pub timestamp : u64
}

impl PacketMeta for S2CStatusPongPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CStatusPongPacket {

    #[inline]
    fn encode_len(&self) -> usize {
        PacketEncode::encode_len(&self.timestamp)
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.timestamp.encode(buf);
    } }

}

impl From<S2CStatusPongPacket> for S2CPackets<'_> {
    #[inline]
    fn from(value : S2CStatusPongPacket) -> Self { Self::Status(value.into()) }
}

impl From<S2CStatusPongPacket> for S2CStatusPackets<'_> {
    #[inline]
    fn from(value : S2CStatusPongPacket) -> Self { Self::Pong(value) }
}

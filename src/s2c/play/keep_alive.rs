//! Clientbound play keep alive packet.


use crate::s2c::{
    S2CPackets,
    play::S2CPlayPackets
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


/// Lets the client know that the connection is still alive.
#[derive(Debug)]
pub struct S2CPlayKeepAlivePacket {
    /// Transaction ID of the keepalive. The client will respond with the same ID.
    pub transaction : u64
}

impl PacketMeta for S2CPlayKeepAlivePacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("keep_alive");
}

unsafe impl PacketEncode for S2CPlayKeepAlivePacket {

    #[inline]
    fn encode_len(&self) -> usize {
        self.transaction.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.transaction.encode(buf);
    } }

}

impl From<S2CPlayKeepAlivePacket> for S2CPackets<'_> {
    #[inline]
    fn from(value : S2CPlayKeepAlivePacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayKeepAlivePacket> for S2CPlayPackets<'_> {
    #[inline]
    fn from(value : S2CPlayKeepAlivePacket) -> Self { Self::KeepAlive(value) }
}

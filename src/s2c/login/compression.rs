//! Clientbound login compression packet.


use crate::s2c::{
    S2CPackets,
    login::S2CLoginPackets
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
use pipeworkmc_data::varint::VarInt;


/// Sets the compression threshold to use for all future communication.
#[derive(Debug)]
pub struct S2CLoginCompressionPacket {
    /// Maximum size of a packet before being compressed.
    pub threshold : u32
}

impl PacketMeta for S2CLoginCompressionPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("login_compression");
}

unsafe impl PacketEncode for S2CLoginCompressionPacket {

    #[inline]
    fn encode_len(&self) -> usize {
        VarInt::<u32>(self.threshold).encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.threshold).encode(buf);
    } }

}

impl From<S2CLoginCompressionPacket> for S2CPackets<'_> {
    #[inline]
    fn from(value : S2CLoginCompressionPacket) -> Self { Self::Login(value.into()) }
}

impl From<S2CLoginCompressionPacket> for S2CLoginPackets<'_> {
    #[inline]
    fn from(value : S2CLoginCompressionPacket) -> Self { Self::Compression(value) }
}

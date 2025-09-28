//! Clientbound status packets.


use crate::s2c::S2CPackets;
use pipeworkmc_codec::{
    encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    meta::PacketMeta
};


pub mod response;
pub mod pong;


/// Clientbound status packets.
#[derive(Debug)]
pub enum S2CStatusPackets<'l> {
    /// Response
    Response(response::S2CStatusResponsePacket<'l>),
    /// Pong
    Pong(pong::S2CStatusPongPacket)
}

impl S2CStatusPackets<'_> {

    /// Returns metadata about this packet.
    pub fn meta(&self) -> (u8, bool,) { match (self) { // TODO: Return a proper structure.
        Self::Response (_) => (response ::S2CStatusResponsePacket ::PREFIX, response ::S2CStatusResponsePacket ::KICK,),
        Self::Pong     (_) => (pong     ::S2CStatusPongPacket     ::PREFIX, pong     ::S2CStatusPongPacket     ::KICK,)
    } }

}

unsafe impl PrefixedPacketEncode for S2CStatusPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Response (packet) => packet.encode_prefixed_len(),
        Self::Pong     (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Response (packet) => packet.encode_prefixed(buf),
        Self::Pong     (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CStatusPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CStatusPackets<'l>) -> Self { Self::Status(value) }
}

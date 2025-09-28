//! Clientbound login packets.


use crate::s2c::S2CPackets;
use pipeworkmc_codec::{
    encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    meta::PacketMeta
};


pub mod disconnect;
pub mod encrypt_request;
pub mod finish;
pub mod compression;


/// Clientbound login packets.
#[derive(Debug)]
pub enum S2CLoginPackets<'l> {
    /// Disconnect
    Disconnect(disconnect::S2CLoginDisconnectPacket),
    /// Encrypt request
    EncryptRequest(encrypt_request::S2CLoginEncryptRequestPacket<'l>),
    /// Finish
    Finish(finish::S2CLoginFinishPacket),
    /// Compression
    Compression(compression::S2CLoginCompressionPacket)
    // TODO: QueryRequest
    // TODO: CookieRequest
}

impl S2CLoginPackets<'_> {

    /// Returns metadata about this packet.
    pub fn meta(&self) -> (u8, bool,) { match (self) { // TODO: Return a proper structure.
        Self::Disconnect     (_) => (disconnect      ::S2CLoginDisconnectPacket     ::PREFIX, disconnect      ::S2CLoginDisconnectPacket     ::KICK,),
        Self::EncryptRequest (_) => (encrypt_request ::S2CLoginEncryptRequestPacket ::PREFIX, encrypt_request ::S2CLoginEncryptRequestPacket ::KICK,),
        Self::Finish         (_) => (finish          ::S2CLoginFinishPacket         ::PREFIX, finish          ::S2CLoginFinishPacket         ::KICK,),
        Self::Compression    (_) => (compression     ::S2CLoginCompressionPacket    ::PREFIX, compression     ::S2CLoginCompressionPacket    ::KICK,)
    } }

}

unsafe impl PrefixedPacketEncode for S2CLoginPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Disconnect     (packet) => packet.encode_prefixed_len(),
        Self::EncryptRequest (packet) => packet.encode_prefixed_len(),
        Self::Finish         (packet) => packet.encode_prefixed_len(),
        Self::Compression    (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Disconnect     (packet) => packet.encode_prefixed(buf),
        Self::EncryptRequest (packet) => packet.encode_prefixed(buf),
        Self::Finish         (packet) => packet.encode_prefixed(buf),
        Self::Compression    (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CLoginPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginPackets<'l>) -> Self { Self::Login(value) }
}

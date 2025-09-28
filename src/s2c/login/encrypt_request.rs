//! Clientbound login encrypt request packet.


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
use pipeworkmc_data::{
    bounded_string::BoundedString,
    redacted::Redacted
};
use std::borrow::Cow;


/// Sends a public key to the client which will be used to send back a shared key.
/// All future communication will be encrypted.
///
/// Also includes a bit more information used in the mojauth process.
#[derive(Debug)]
pub struct S2CLoginEncryptRequestPacket<'l> {
    /// ID of the server, used in the mojauth process.
    pub server_id       : BoundedString<20>,
    /// Public key to use when sending back a shared key.
    pub public_key      : Redacted<Cow<'l, [u8]>>,
    /// Verify token, used to ensure that the key exchange was successful.
    pub verify_token    : [u8; 4],
    /// Whether to enable mojauth.
    pub mojauth_enabled : bool
}

impl PacketMeta for S2CLoginEncryptRequestPacket<'_> {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginEncryptRequestPacket<'_> {

    fn encode_len(&self) -> usize {
        self.server_id.encode_len()
        + unsafe { self.public_key.as_ref() }.encode_len()
        + self.verify_token.encode_len()
        + self.mojauth_enabled.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.server_id.encode(buf);
        self.public_key.as_ref().encode(buf);
        self.verify_token.as_slice().encode(buf);
        self.mojauth_enabled.encode(buf);
    } }

}

impl<'l> From<S2CLoginEncryptRequestPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginEncryptRequestPacket<'l>) -> Self { Self::Login(value.into()) }
}

impl<'l> From<S2CLoginEncryptRequestPacket<'l>> for S2CLoginPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginEncryptRequestPacket<'l>) -> Self { Self::EncryptRequest(value) }
}

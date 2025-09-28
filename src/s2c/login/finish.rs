//! Clientbound login finish packet.


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
use pipeworkmc_data::profile::AccountProfile;


/// Informs the client that future packets will be in the configuration state.
#[derive(Debug)]
pub struct S2CLoginFinishPacket {
    /// The profile that the server recognises the client as.
    pub profile : AccountProfile
}


impl PacketMeta for S2CLoginFinishPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::S2C;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginFinishPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.profile.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.profile.encode(buf);
    } }

}

impl<'l> From<S2CLoginFinishPacket> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginFinishPacket) -> Self { Self::Login(value.into()) }
}

impl<'l> From<S2CLoginFinishPacket> for S2CLoginPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginFinishPacket) -> Self { Self::Finish(value) }
}

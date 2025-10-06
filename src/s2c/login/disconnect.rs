//! Clientbound login disconnect packet.


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
use pipeworkmc_data::text::Text;
use serde_json::to_string as to_json_string;


/// Kicks the player from the server with a reason.
#[derive(Debug)]
pub struct S2CLoginDisconnectPacket {
    reason_json : String
}

impl PacketMeta for S2CLoginDisconnectPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("login_disconnect");
    const KICK   : bool        = true;
}

unsafe impl PacketEncode for S2CLoginDisconnectPacket {

    #[inline]
    fn encode_len(&self) -> usize {
        self.reason_json.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.reason_json.encode(buf);
    } }

}

impl<'l> From<S2CLoginDisconnectPacket> for S2CPackets<'l> {
    #[inline]
    fn from(value : S2CLoginDisconnectPacket) -> Self { Self::Login(value.into()) }
}

impl<'l> From<S2CLoginDisconnectPacket> for S2CLoginPackets<'l> {
    #[inline]
    fn from(value : S2CLoginDisconnectPacket) -> Self { Self::Disconnect(value) }
}


impl<'l, S> From<S> for S2CLoginDisconnectPacket
where
    S : Into<&'l Text>
{
    #[inline]
    fn from(value : S) -> Self {
        Self { reason_json : to_json_string(value.into()).unwrap() }
    }
}

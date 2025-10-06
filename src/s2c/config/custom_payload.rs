//! Clientbound configuration custom payload packet.


use crate::s2c::{
    S2CPackets,
    config::S2CConfigPackets
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
use pipeworkmc_data::channel_data::ChannelData;


/// Custom data sent to the client.
#[derive(Debug)]
pub struct S2CConfigCustomPayloadPacket<'l> {
    /// Custom data.
    pub data : ChannelData<'l>
}


impl PacketMeta for S2CConfigCustomPayloadPacket<'_> {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("custom_payload");
}

unsafe impl PacketEncode for S2CConfigCustomPayloadPacket<'_> {

    #[inline]
    fn encode_len(&self) -> usize {
        self.data.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.data.encode(buf);
    } }

}

impl<'l> From<S2CConfigCustomPayloadPacket<'l>> for S2CPackets<'l> {
    #[inline]
    fn from(value : S2CConfigCustomPayloadPacket<'l>) -> Self { Self::Config(value.into()) }
}

impl<'l> From<S2CConfigCustomPayloadPacket<'l>> for S2CConfigPackets<'l> {
    #[inline]
    fn from(value : S2CConfigCustomPayloadPacket<'l>) -> Self { Self::CustomPayload(value) }
}

//! Serverbound configuration custom payload packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::channel_data::{
    ChannelData,
    ChannelDataDecodeError
};


/// Custom data sent to the server.
#[derive(Debug)]
pub struct C2SConfigCustomPayloadPacket {
    /// Custom data.
    pub data : ChannelData<'static>
}

impl PacketMeta for C2SConfigCustomPayloadPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("custom_payload");
}

impl PacketDecode for C2SConfigCustomPayloadPacket {
    type Error = ChannelDataDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        data : <_>::decode(iter)?,
    }) }
}

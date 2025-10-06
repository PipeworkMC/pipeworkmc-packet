//! Serverbound play custom payload packet.


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
pub struct C2SPlayCustomPayloadPacket {
    /// Custom data.
    pub data : ChannelData<'static>
}

impl PacketMeta for C2SPlayCustomPayloadPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("custom_payload");
}

impl PacketDecode for C2SPlayCustomPayloadPacket {
    type Error = ChannelDataDecodeError;

    fn decode<I>(buf : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        data : <_>::decode(buf)?,
    }) }
}

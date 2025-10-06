//! Serverbound configuration client info packet.


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
use pipeworkmc_data::client_info::{
    ClientInfo,
    ClientInfoDecodeError
};


/// Informs the server that some client settings have been updated.
#[derive(Debug)]
pub struct C2SConfigClientInfoPacket {
    /// New client settings.
    pub info : ClientInfo
}

impl PacketMeta for C2SConfigClientInfoPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("client_information");
}

impl PacketDecode for C2SConfigClientInfoPacket {
    type Error = ClientInfoDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        info : <_>::decode(iter)?
    }) }
}

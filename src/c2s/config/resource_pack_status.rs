//! Serverbound configuration pong packet.


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
use pipeworkmc_data::pack_status::{
    PackStatus,
    PackStatusDecodeError
};


/// Resource pack load status.
#[derive(Debug)]
pub struct C2SConfigResourcePackStatusPacket {
    /// Current status.
    pub status : PackStatus
}

impl PacketMeta for C2SConfigResourcePackStatusPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("resource_pack");
}

impl PacketDecode for C2SConfigResourcePackStatusPacket {
    type Error = PackStatusDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        status : <_>::decode(iter)?
    }) }
}

//! Serverbound login finish acknowledged packet.


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


/// The client has recognised that login has been completed.
#[derive(Debug)]
pub struct C2SLoginFinishAcknowledgedPacket;

impl PacketMeta for C2SLoginFinishAcknowledgedPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("login_acknowledged");
}

impl PacketDecode for C2SLoginFinishAcknowledgedPacket {
    type Error = !;

    #[inline]
    fn decode<I>(_ : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self) }
}

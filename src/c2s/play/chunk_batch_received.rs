//! Serverbound play chunk batch received packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


/// Tells the server what rate to send chunks at.
#[derive(Debug)]
pub struct C2SPlayChunkBatchReceivedPacket {
    /// The desired rate.
    pub desired_chunks_per_tick : f32
}

impl PacketMeta for C2SPlayChunkBatchReceivedPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("chunk_batch_received");
}

impl PacketDecode for C2SPlayChunkBatchReceivedPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        desired_chunks_per_tick : <_>::decode(iter)?
    }) }
}

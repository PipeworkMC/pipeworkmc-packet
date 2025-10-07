//! Serverbound play acknowledge teleport packet.


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
    },
    varint::{
        VarInt,
        VarIntDecodeError
    }
};
use pipeworkmc_data::block_pos::BlockPos;
use core::fmt::{ self, Display, Formatter };


/// Lets the server know that a previously received teleport has been received.
#[derive(Debug)]
pub struct C2SPlayQueryBlockEntityTagPacket {
    /// Transaction ID of the teleport. The server previously sent the value to use.
    pub transaction : u32,
    /// Location of the block to check.
    pub block       : BlockPos
}

impl PacketMeta for C2SPlayQueryBlockEntityTagPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("block_entity_tag_query");
}

impl PacketDecode for C2SPlayQueryBlockEntityTagPacket {
    type Error = C2SPlayQueryBlockEntityTagDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        transaction : <VarInt<u32>>::decode(iter).map_err(C2SPlayQueryBlockEntityTagDecodeError::Transaction)?.0,
        block       : <_>::decode(iter).map_err(C2SPlayQueryBlockEntityTagDecodeError::Location)?
    }) }
}


/// Returned by packet decoders when a `C2SPlayQueryBlockEntityTagPacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SPlayQueryBlockEntityTagDecodeError {
    /// The transaction ID failed to decode.
    Transaction(VarIntDecodeError),
    /// The block location failed to decode.
    Location(IncompleteDecodeError)
}
impl Display for C2SPlayQueryBlockEntityTagDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Transaction(err) => write!(f, "transaction {err}"),
        Self::Location(err)    => write!(f, "location {err}")
    } }
}

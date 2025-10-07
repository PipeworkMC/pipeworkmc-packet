//! Serverbound play chat message packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        string::StringDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::varint::VarInt;
use core::fmt::{ self, Display, Formatter };


/// The player sent a chat message.
///
/// **Note**: Pipework does not support chat signatures.
#[derive(Debug)]
pub struct C2SPlayChatMessagePacket {
    /// The message that was sent.
    pub message : String
}

impl PacketMeta for C2SPlayChatMessagePacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("chat");
}

impl PacketDecode for C2SPlayChatMessagePacket {
    type Error = C2SPlayChatMessageDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let message = <_>::decode(iter).map_err(C2SPlayChatMessageDecodeError::Message)?;
        iter.skip(size_of::<u64>()).map_err(|_| C2SPlayChatMessageDecodeError::Signature)?; // Timestamp
        iter.skip(size_of::<u64>()).map_err(|_| C2SPlayChatMessageDecodeError::Signature)?; // Salt
        if (iter.read().map_err(|_| C2SPlayChatMessageDecodeError::Signature)? != 0) {
            iter.skip(256).map_err(|_| C2SPlayChatMessageDecodeError::Signature)?; // Signature
        }
        _ = VarInt::<u32>::decode(iter).map_err(|_| C2SPlayChatMessageDecodeError::Signature)?; // Message count
        iter.skip(3).map_err(|_| C2SPlayChatMessageDecodeError::Signature)?; // Acknowledged
        iter.skip(1).map_err(|_| C2SPlayChatMessageDecodeError::Signature)?; // Checksum
        Ok(Self { message }) }
}


/// Returned by packet decoders when a `C2SPlayChatMessagePacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SPlayChatMessageDecodeError {
    /// The message failed to decode.
    Message(StringDecodeError),
    /// The signature was not the correct size.
    Signature
}
impl Display for C2SPlayChatMessageDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Message(err) => write!(f, "message {err}"),
        Self::Signature    => write!(f, "signature bad length")
    } }
}

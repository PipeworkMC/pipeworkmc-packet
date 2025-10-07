//! Serverbound play signed chat command packet.


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


/// The player ran a command..
///
/// **Note**: Pipework does not support chat signatures.
#[derive(Debug)]
pub struct C2SPlaySignedChatCommandPacket {
    /// The command to run, without the `/`.
    pub command : String
}

impl PacketMeta for C2SPlaySignedChatCommandPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("chat_command_signed");
}

impl PacketDecode for C2SPlaySignedChatCommandPacket {
    type Error = C2SPlaySignedChatCommandDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        let command = <_>::decode(iter).map_err(C2SPlaySignedChatCommandDecodeError::Command)?;
        iter.skip(size_of::<u64>()).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Timestamp
        iter.skip(size_of::<u64>()).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Salt
        for _ in 0..VarInt::<u32>::decode(iter).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?.0 { // Arguments
            let len = VarInt::<u32>::decode(iter).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?.0 as usize;
            iter.skip(len).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Argument name
            iter.skip(256).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Signature
        }
        _ = VarInt::<u32>::decode(iter).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Message count
        iter.skip(3).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Acknowledged
        iter.skip(1).map_err(|_| C2SPlaySignedChatCommandDecodeError::Signature)?; // Checksum
        Ok(Self { command }) }
}


/// Returned by packet decoders when a `C2SPlaySignedChatCommandPacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SPlaySignedChatCommandDecodeError {
    /// The command failed to decode.
    Command(StringDecodeError),
    /// The signature was not the correct size.
    Signature
}
impl Display for C2SPlaySignedChatCommandDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Command(err) => write!(f, "command {err}"),
        Self::Signature    => write!(f, "signature bad length")
    } }
}

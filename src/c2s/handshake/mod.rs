//! Serverbound handshaking packets.


pub mod intention;


super::packet_group!(
    "handshaking" C2SHandshakePackets,
    C2SHandshakeDecodeError,
    {
        "intention" Intention => intention::C2SHandshakeIntentionPacket
    }
);

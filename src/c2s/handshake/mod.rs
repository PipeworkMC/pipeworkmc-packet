//! Serverbound handshaking packets.


pub mod intention;


super::packet_group!(
    "handshaking" C2SHandshakePackets,
    C2SHandshakeDecodeError,
    {
        "intention" Intention => intention::C2SHandshakeIntentionPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/c2s_handshake.rs");

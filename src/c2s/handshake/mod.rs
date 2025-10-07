//! Serverbound handshaking packets.


pub mod intent;


super::packet_group!(
    "handshaking" C2SHandshakePackets,
    C2SHandshakeDecodeError,
    {
        "intent" Intent => intent::C2SHandshakeIntentPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/c2s_handshake.rs");

//! Clientbound status packets.


pub mod response;
pub mod pong;


super::packet_group!(
    "status" Status S2CStatusPackets<'l>, meta,
    {
        "response" Response => response ::S2CStatusResponsePacket<'l>,
        "pong"     Pong     => pong     ::S2CStatusPongPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/s2c_status.rs");

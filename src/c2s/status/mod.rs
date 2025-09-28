//! Serverbound status packets.


pub mod request;
pub mod ping;


super::packet_group!(
    "status" C2SStatusPackets,
    C2SStatusDecodeError,
    {
        "request" Request => request ::C2SStatusRequestPacket,
        "ping"    Ping    => ping    ::C2SStatusPingPacket
    }
);

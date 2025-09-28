//! Serverbound login packets.


pub mod start;
pub mod encrypt_response;
// TODO: custom_query_response
pub mod finish_acknowledged;
// TODO: cookie_response


super::packet_group!(
    "login" C2SLoginPackets,
    C2SLoginDecodeError,
    {
        "start"               Start              => start               ::C2SLoginStartPacket,
        "encrypt response"    EncryptResponse    => encrypt_response    ::C2SLoginEncryptResponsePacket,
        "finish acknowledged" FinishAcknowledged => finish_acknowledged ::C2SLoginFinishAcknowledgedPacket
    }
);

//! Serverbound login packets.


pub mod start;
pub mod encrypt_response;
pub mod custom_query_response;
pub mod finish_acknowledged;
pub mod cookie_response;


super::packet_group!(
    "login" C2SLoginPackets,
    C2SLoginDecodeError,
    {
        "start"                 Start               => start                 ::C2SLoginStartPacket,
        "encrypt response"      EncryptResponse     => encrypt_response      ::C2SLoginEncryptResponsePacket,
        "custom query response" CustomQueryResponse => custom_query_response ::C2SLoginCustomQueryResponsePacket,
        "finish acknowledged"   FinishAcknowledged  => finish_acknowledged   ::C2SLoginFinishAcknowledgedPacket,
        "cookie response"       CookieResponse      => cookie_response       ::C2SLoginCookieResponsePacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/c2s_login.rs");

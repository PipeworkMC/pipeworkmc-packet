//! Clientbound login packets.


pub mod disconnect;
pub mod encrypt_request;
pub mod finish;
pub mod compression;
// TODO: custom_query
// TODO: cookie_request


super::packet_group!(
    "login" Login S2CLoginPackets<'l>, meta,
    {
        "disconnect"      Disconnect     => disconnect      ::S2CLoginDisconnectPacket,
        "encrypt_request" EncryptRequest => encrypt_request ::S2CLoginEncryptRequestPacket<'l>,
        "finish"          Finish         => finish          ::S2CLoginFinishPacket,
        "compression"     Compression    => compression     ::S2CLoginCompressionPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/s2c_login.rs");

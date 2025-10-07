//! Clientbound packets.


use pipeworkmc_codec::{
    encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    meta::{
        PacketMeta,
        PacketMetadata
    }
};


pub mod status;
pub mod login;
pub mod config;
pub mod play;


/// Clientbound packets.
#[derive(Debug)]
pub enum S2CPackets<'l> {
    /// Clientbound status packets.
    Status(status::S2CStatusPackets<'l>),
    /// Clientbound login packets.
    Login(login::S2CLoginPackets<'l>),
    /// Clientbound configuration packets.
    Config(config::S2CConfigPackets<'l>),
    /// Clientbound play packets.
    Play(play::S2CPlayPackets<'l>)
}

impl S2CPackets<'_> {

    /// Returns metadata about this packet.
    pub fn meta(&self) -> PacketMetadata { match (self) {
        Self::Status (packet) => packet.meta(),
        Self::Login  (packet) => packet.meta(),
        Self::Config (packet) => packet.meta(),
        Self::Play   (packet) => packet.meta()
    } }

}

unsafe impl PrefixedPacketEncode for S2CPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        S2CPackets::Status (packet) => packet.encode_prefixed_len(),
        S2CPackets::Login  (packet) => packet.encode_prefixed_len(),
        S2CPackets::Config (packet) => packet.encode_prefixed_len(),
        S2CPackets::Play   (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        S2CPackets::Status (packet) => packet.encode_prefixed(buf),
        S2CPackets::Login  (packet) => packet.encode_prefixed(buf),
        S2CPackets::Config (packet) => packet.encode_prefixed(buf),
        S2CPackets::Play   (packet) => packet.encode_prefixed(buf)
    } } }

}


macro packet_group(
    $state:tt $statevar:ident $group:ident $(<$lt:lifetime>)?, $meta:ident,
    { $( $name:tt $variant:ident => $ty:ty ),* $(,)? }
) {

    #[doc = concat!("Clientbound", $state, "packets.")]
    #[derive(Debug)]
    pub enum $group$(<$lt>)? { $(
        #[doc = $name]
        $variant( $ty ),
    )* }

    impl$(<$lt>)? $group$(<$lt>)? {
        /// Returns metadata about this packet.
        pub fn $meta(&self) -> PacketMetadata { match (self) { $(
            Self::$variant (_) => <$ty as PacketMeta>::metadata(),
        )* } }
    }

    unsafe impl$(<$lt>)? PrefixedPacketEncode for $group$(<$lt>)? {

        fn encode_prefixed_len(&self) -> usize { match (self) { $(
            Self::$variant (packet) => packet.encode_prefixed_len(),
        )* } }

        unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) { $(
            Self::$variant (packet) => packet.encode_prefixed(buf),
        )* } } }

    }

    impl<'l> From<$group$(<'l> ${ignore($lt)})?> for S2CPackets<'l> {
        #[inline]
        fn from(value : $group$(<'l> ${ignore($lt)})?) -> Self {
            Self::$statevar(value)
        }
    }

}

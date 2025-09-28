//! Clientbound configuration registry data packet.


use crate::s2c::{
    S2CPackets,
    config::S2CConfigPackets
};
use pipeworkmc_codec::{
    encode::{
        PacketEncode,
        EncodeBuf,
        slice::UnprefixedSlice
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::{
    ident::Ident,
    registry_entry::{
        RegistryEntry,
        RegistryEntryType
    }
};
use core::fmt::Debug;
use std::borrow::Cow;


/// Informs the client of entries in a non-static registry.
#[derive(Debug)]
pub struct S2CConfigRegistryDataPacket<'l> {
    registry : Ident,
    entries  : Cow<'l, [(Ident, Option<UnprefixedSlice<'l, u8>>,)]>
}


impl PacketMeta for S2CConfigRegistryDataPacket<'_> {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x07; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigRegistryDataPacket<'_> {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.registry.encode_len()
        + self.entries.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.registry.encode(buf);
        self.entries.encode(buf);
    } }

}

impl<'l> From<S2CConfigRegistryDataPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigRegistryDataPacket<'l>) -> Self { Self::Config(value.into()) }
}

impl<'l> From<S2CConfigRegistryDataPacket<'l>> for S2CConfigPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigRegistryDataPacket<'l>) -> Self { Self::RegistryData(value) }
}


impl<'l, I, T> From<I> for S2CConfigRegistryDataPacket<'l>
where
    I : IntoIterator<Item = &'l RegistryEntry<T>>,
    T : RegistryEntryType + 'l
{
    fn from(entries : I) -> Self { Self {
        registry : T::REGISTRY_ID,
        entries  : Cow::Owned(entries.into_iter().map(|entry| {
            let mut buf     = Vec::new();
            let     is_some = entry.data.to_network_nbt(&mut buf);
            (entry.id.clone(), is_some.then(|| UnprefixedSlice::from(buf)),)
        }).collect::<Vec<_>>())
    } }
}

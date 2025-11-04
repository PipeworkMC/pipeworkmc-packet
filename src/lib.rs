#![doc = include_str!("../README.md")]


#![feature(

    // Syntax
    decl_macro

)]


use crate::meta::*;
use pipeworkmc_data::Minecraft;
use netzer::prelude::*;


pub mod meta;

pub mod c2s;
pub mod s2c;


macro packet_group(
    $boundname:tt $statename:tt =>
    $vis:vis enum $ident:ident $(<$($lt:lifetime),*$(,)?>)? {
        $( $variantident:ident( $variantty:ty ) ),*$(,)?
    }
) {

    #[derive(Debug, Clone)]
    $vis enum $ident $(<$($lt,)*>)? {
        $( $variantident( $variantty ), )*
    }

    impl $(<$($lt,)*>)? NetEncode<Minecraft> for $ident $(<$($lt,)*>)? {
        async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
            match (self) {
                $( Self::$variantident(value) => {
                    w.write_all(&[<$variantty as Packet>::PREFIX]).await?;
                    <$variantty as NetEncode<Minecraft>>::encode(value, w).await
                }, )*
            }
        }
    }
    impl $(<$($lt,)*>)? NetDecode<Minecraft> for $ident $(<$($lt,)*>)? {
        async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
            let mut buf = [0u8; 1];
            r.read_exact(&mut buf).await?;
            Ok(match (buf[0]) {
                $( <$variantty as Packet>::PREFIX => {
                    Self::$variantident(<$variantty as NetDecode<Minecraft>>::decode(r).await?)
                }, )*
                x => { return Err(format!(concat!("invalid id {} for ", $boundname, " ", $statename, " packet"), x).into()); },
            })
        }
    }

}

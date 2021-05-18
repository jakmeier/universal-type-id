use core::any::Any;
use std::convert::TryInto;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub const MAX_UTI_BYTES: usize = 16;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UniversalTypeId {
    bytes: [u8; MAX_UTI_BYTES],
}
pub trait UniversalType: Any {
    /// Raw bytes which are the result of the universal type hash
    const UNIVERSAL_TYPE_ID_BYTES: [u8; MAX_UTI_BYTES];

    /// A type id as  a hash over the type name and fields
    fn universal_type_id(&self) -> UniversalTypeId {
        UniversalTypeId::of::<Self>()
    }
}

impl UniversalTypeId {
    pub fn of<T: ?Sized + UniversalType>() -> UniversalTypeId {
        UniversalTypeId {
            bytes: T::UNIVERSAL_TYPE_ID_BYTES,
        }
    }
    pub fn as_u16(&self) -> u16 {
        u16::from_be_bytes(self.bytes[0..2].try_into().unwrap())
    }
    pub fn as_u32(&self) -> u32 {
        u32::from_be_bytes(self.bytes[0..4].try_into().unwrap())
    }
    pub fn as_u64(&self) -> u64 {
        u64::from_be_bytes(self.bytes[0..8].try_into().unwrap())
    }
    #[cfg(feature = "integer128")]
    pub fn as_u128(&self) -> u128 {
        u128::from_be_bytes(self.bytes[0..16].try_into().unwrap())
    }
}

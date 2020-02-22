use crate::Error;
use ssz::{Decode, Encode};
use tree_hash::TreeHash;

pub const PUBLIC_KEY_BYTES_LEN: usize = 48;

pub trait TPublicKey: Sized {
    fn zero() -> Self;

    fn add_assign(&mut self, other: &Self);

    fn serialize(&self) -> [u8; PUBLIC_KEY_BYTES_LEN];

    fn deserialize(bytes: &[u8]) -> Result<Self, Error>;
}

pub struct PublicKey<T: TPublicKey> {
    point: T,
}

impl<T: TPublicKey> PublicKey<T> {
    pub fn zero() -> Self {
        Self { point: T::zero() }
    }

    pub fn add_assign(&mut self, other: &Self) {
        self.point.add_assign(&other.point)
    }

    pub fn serialize(&self) -> [u8; PUBLIC_KEY_BYTES_LEN] {
        self.point.serialize()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            point: T::deserialize(bytes)?,
        })
    }
}

impl<T: TPublicKey> Encode for PublicKey<T> {
    impl_ssz_encode!(PUBLIC_KEY_BYTES_LEN);
}

impl<T: TPublicKey> Decode for PublicKey<T> {
    impl_ssz_decode!(PUBLIC_KEY_BYTES_LEN);
}

impl<T: TPublicKey> TreeHash for PublicKey<T> {
    impl_tree_hash!(PUBLIC_KEY_BYTES_LEN);
}

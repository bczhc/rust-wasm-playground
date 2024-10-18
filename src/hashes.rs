use digest::Digest;
use strum::EnumString;

pub fn ripemd160(data: &[u8]) -> [u8; 160 / 8] {
    ripemd::Ripemd160::digest(data).into()
}

pub fn sha256(data: &[u8]) -> [u8; 256 / 8] {
    sha2::Sha256::digest(data).into()
}

pub fn sha256d(data: &[u8]) -> [u8; 256 / 8] {
    sha256(&sha256(data))
}

/// The "SHA160" hash function in Bitcoin
///
/// SHA160 = RIPEMD160(SHA256(x))
pub fn sha160(data: &[u8]) -> [u8; 160 / 8] {
    ripemd160(&sha256(data))
}

#[derive(Debug, Copy, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum DigestType {
    Ripemd160,
    Sha256,
    Sha256d,
    Sha160,
}

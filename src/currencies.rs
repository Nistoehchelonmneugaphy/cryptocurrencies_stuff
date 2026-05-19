use crate::atomic_unit::{AtomicUnit, CryptocurrencyConfig};


#[derive(Clone, Debug)]
pub struct XMR;
impl CryptocurrencyConfig for XMR {
    const ATOMIC_PER_UNIT: u64 = 1_000_000_000_000;
    const SYMBOL: &'static str = "XMR";
}

pub type XMRAtomicUnit = AtomicUnit<XMR>;

#[derive(Clone, Debug)]
pub struct LTC;
impl CryptocurrencyConfig for LTC {
    const ATOMIC_PER_UNIT: u64 = 100_000_000;
    const SYMBOL: &'static str = "LTC";
}
pub type LTCAtomicUnit = AtomicUnit<LTC>;
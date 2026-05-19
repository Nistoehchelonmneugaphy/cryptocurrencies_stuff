pub mod atomic_unit;
pub mod currencies;
pub mod derive_ltc;

#[cfg(test)]
mod tests {
    use crate::derive_ltc::{derive_ltc_bip84_address};

    #[test]
    pub fn test_ltc() {
        let mnemonic =
            "stairs grace visit maple author pattern pumpkin erosion chimney learn quick cabin";

        let max = 10;
        let mut index = 0;
        for li in (0..=3).skip(1) {
            index = 0;
            println!("loop {li}");
            loop {
                let Ok(addr) = derive_ltc_bip84_address(mnemonic, index) else {
                    assert_eq!(1, 2);
                    break;
                };
                println!("{index} {addr}");
                index += 1;
                if index > max {
                    break;
                }
            }
        }
    }
}

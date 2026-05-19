use bip39::Mnemonic;

pub fn generate_mnemonic(word_count: u32) -> String {
    assert!(
        word_count == 12 || word_count == 24,
        "word_count must be 12 or 24, got {word_count}"
    );
    Mnemonic::generate(word_count as usize)
        .expect("valid word count")
        .to_string()
}

//
fn has_character_count(s: &str, char_count: u64) -> bool {
    s.chars().count() as u64 == char_count
}

// P2PKH ----------------------------------------------------------------------------------------
// PUB KEY HASH
// P2PKH addresses are 33-34 characters long, use Base58Check encoding, and (on mainnet) start with "1".
//
// Pay to Pubkey Hash (P2PKH) addresses start with 1 and represent a single public key.
fn is_p2pkh(address: &String) -> bool {
    let p2pkh_leading_symbols = vec![
        "1", // mainnet
        "m", //testnet
        "n", //testnet
    ];

    let has_p2pkh_leading_symbol = p2pkh_leading_symbols
        .iter()
        .any(|leading_symbol| address.starts_with(leading_symbol));
    let has_correct_char_count =
        has_character_count(address, 33) || has_character_count(address, 34);
    has_correct_char_count && has_p2pkh_leading_symbol
}
// ----------------------------------------------------------------------------------------

// P2SH ----------------------------------------------------------------------------------------
// SCRIPT HASH
// P2SH addresses are 34 characters long, use Base58Check encoding, and (on mainnet) start with "3".
//
// Pay to Script Hash (P2SH) addresses present a much more general validation scheme that can encode an arbitrary script. These start with "3" and are common for multi-signature setups.
fn is_p2sh(address: &String) -> bool {
    let p2sh_leading_symbols = vec![
        "2", // mainnet
        "3", //testnet
    ];

    let has_p2sh_leading_symbol = p2sh_leading_symbols
        .iter()
        .any(|leading_symbol| address.starts_with(leading_symbol));
    let has_correct_char_count =
        // mainnet
        has_character_count(address, 34) 
        // testnet
        || has_character_count(address, 35); 
    has_correct_char_count && has_p2sh_leading_symbol
}
// ----------------------------------------------------------------------------------------

//P2SH_P2WPKH ----------------------------------------------------------------------------------
// Pay To Witness Public Key Hash Wrapped In P2SH
// Allows non-SegWit wallets to generate a SegWit transaction. Allows non-SegWit client accept SegWit transaction.
// Source: https://www.btcschools.net/bitcoin/bitcoin_script_p2sh_p2wpkh.php
//
fn could_be_p2sh_p2wpkh(address: &String) -> bool {
    is_p2sh(address)
}
//----------------------------------------------------------------------------------------

//P2SH_P2WSH ----------------------------------------------------------------------------------
// NESTED SEGWIT
// Pay To Witness Script Hash Wrapped In P2SH
// Allows non-SegWit wallets to generate a SegWit transaction. Allows non-SegWit client accept SegWit transaction.
// Source: https://www.btcschools.net/bitcoin/bitcoin_script_p2sh_p2wsh.php
//
fn could_be_p2sh_p2wsh(address: &String) -> bool {
    is_p2sh(address)
}
//----------------------------------------------------------------------------------------

// P2WPKH ----------------------------------------------------------------------------------------
// SEGWIT v0 PUBKEY HASH
// P2WPKH addresses are 42 characters long, use bech32 encoding, and (on mainnet) start with "bc1q".
//
// Pay to Witness Pubkey Hash (P2WPKH) addresses are version 0 segregated witness (SegWit) programs and behave similarly to P2PKH. All version 0 SegWit addresses start with "bc1q"
fn is_p2wpkh(address: &String) -> bool {
    let p2wpkh_leading_symbols = vec![
        "bc1q", // mainnet
        "tb1q", //testnet
    ];

    let has_p2wpkh_leading_symbol = p2wpkh_leading_symbols
        .iter()
        .any(|leading_symbol| address.starts_with(leading_symbol));
    has_character_count(address, 42) && has_p2wpkh_leading_symbol
}
// ----------------------------------------------------------------------------------------

// P2WSH ----------------------------------------------------------------------------------------
// SegWit v0 Script Hash
// P2WSH addresses are 62 characters long, use bech32 encoding, and (on mainnet) start with "bc1q".
// Pay to Witness Script Hash (P2WSH) addresses are version 0 segregated witness (SegWit) programs and behave similarly to P2SH. All version 0 SegWit addresses start with "bc1q"
//
fn is_p2wsh(address: &String) -> bool {
    let p2wsh_leading_symbols = vec![
        "bc1q", // mainnet
        "tb1q", //testnet
    ];

    let has_p2wsh_leading_symbol = p2wsh_leading_symbols
        .iter()
        .any(|leading_symbol| address.starts_with(leading_symbol));
    has_character_count(address, 62) && has_p2wsh_leading_symbol
}
//
// ----------------------------------------------------------------------------------------

// P2TR ----------------------------------------------------------------------------------------
// Pay to taproot
// P2TR addresses are 62 characters long, use bech32m encoding, and (on mainnet) start with "bc1p".

fn is_p2tr(address: &String) -> bool {
    let p2tr_leading_symbols = vec![
        "bc1p", // mainnet
        "tb1p", //testnet
    ];

    let has_p2tr_leading_symbol = p2tr_leading_symbols
        .iter()
        .any(|leading_symbol| address.starts_with(leading_symbol));
    has_character_count(address, 62) && has_p2tr_leading_symbol
}
// ----------------------------------------------------------------------------------------

pub fn is_legacy(address: &String) -> bool {
    is_p2pkh(address)
}

pub fn is_nested_segwit(address: &String) -> bool {
    is_p2sh(address)
}
// Nested segwit is also called wrapped segwit
pub fn is_wrapped_segwit(address: &String) -> bool {
    is_nested_segwit(address)
}

pub fn is_segwit_native(address: &String) -> bool {
    is_p2wpkh(address) || is_p2wsh(address)
}

pub fn is_segwit_v0(address: &String) -> bool {
    is_segwit_native(address) || is_p2sh(address)
}
pub fn is_segwit_v1(address: &String) -> bool {
    is_taproot(address)
}

pub fn is_taproot(address: &String) -> bool {
    is_p2tr(address)
}

// TODO: Add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_legacy_works() {
        let legacy_address = "1J9uwBYepTm5737RtzkSEePTevGgDGLP5S".to_string();
        let nested_segwit_address = "37u4L57bLqZ8NL9bs1GNX2x52KxviDfvPp".to_string();
        let native_segwit_address = "bc1qfvmj8jse4r7203mrchfyt24sjcpna3s2y35ylp".to_string();
        let taproot_address =
            "bc1p8denc9m4sqe9hluasrvxkkdqgkydrk5ctxre5nkk4qwdvefn0sdsc6eqxe".to_string();

        assert_eq!(is_legacy(&legacy_address), true);
        assert_eq!(is_legacy(&nested_segwit_address), false);
        assert_eq!(is_legacy(&native_segwit_address), false);
        assert_eq!(is_legacy(&taproot_address), false);

        assert_eq!(is_nested_segwit(&nested_segwit_address), true);
        assert_eq!(is_nested_segwit(&legacy_address), false);
        assert_eq!(is_nested_segwit(&native_segwit_address), false);
        assert_eq!(is_nested_segwit(&taproot_address), false);

        assert_eq!(is_segwit_native(&native_segwit_address), true);
        assert_eq!(is_segwit_native(&legacy_address), false);
        assert_eq!(is_segwit_native(&nested_segwit_address), false);
        assert_eq!(is_segwit_native(&taproot_address), false);

        assert_eq!(is_segwit_v0(&nested_segwit_address), true);
        assert_eq!(is_segwit_v0(&native_segwit_address), true);
        assert_eq!(is_segwit_v0(&taproot_address), false);

        assert_eq!(is_segwit_v1(&nested_segwit_address), false);
        assert_eq!(is_segwit_v1(&native_segwit_address), false);
        assert_eq!(is_segwit_v1(&taproot_address), true);

        assert_eq!(is_taproot(&taproot_address), true);
        assert_eq!(is_taproot(&legacy_address), false);
        assert_eq!(is_taproot(&nested_segwit_address), false);
        assert_eq!(is_taproot(&native_segwit_address), false);
    }
}

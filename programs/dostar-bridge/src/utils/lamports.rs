pub fn calculate_lamports(amount: u64, decimals: u8) -> u64 {
    amount.checked_mul(10u64.pow(decimals as u32)).unwrap()
}

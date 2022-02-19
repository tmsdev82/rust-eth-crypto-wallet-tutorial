use std::time::{SystemTime, UNIX_EPOCH};
use web3::types::U256;

pub fn get_timestamp(time: SystemTime) -> u64 {
    let since_epoch = time.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_secs()
}

pub fn get_current_timestamp() -> u64 {
    get_timestamp(SystemTime::now())
}

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

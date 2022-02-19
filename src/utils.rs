use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp(time: SystemTime) -> u64 {
    let since_epoch = time.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_secs()
}

pub fn get_current_timestamp() -> u64 {
    get_timestamp(SystemTime::now())
}

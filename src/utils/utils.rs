use chrono::NaiveDateTime;

pub fn from_epoch(timestamp: i64) -> NaiveDateTime {
    let ns: u32 = (timestamp % 1_000_000_000) as u32;
    let sec: i64 = timestamp / 1_000_000_000;
    NaiveDateTime::from_timestamp(sec, ns)
}

pub fn from_ms_ns(ms: u64, us: u64) -> NaiveDateTime {
    let timestamp: i64 = (ms * 1_000_000 + us) as i64;
    let ns: u32 = (timestamp % 1_000_000_000) as u32;
    let sec: i64 = timestamp / 1_000_000_000;
    NaiveDateTime::from_timestamp(sec, ns)
}

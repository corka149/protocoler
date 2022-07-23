//! Module for helper functions.

use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn tmp_csv_path() -> Result<PathBuf, SystemTimeError> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp = duration.as_secs();

    let mut temp_path = env::temp_dir();
    let protocol_file = format!("{}_protocol.csv", timestamp);
    temp_path.push(protocol_file);

    Ok(temp_path)
}

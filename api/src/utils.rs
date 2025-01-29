use chrono::{DateTime, Utc};
use sonyflake::{decompose, Sonyflake};
use tonic::Status;

pub fn next_id(sf: &Sonyflake) -> Result<(i64, i64, u16), Status> {
    match sf.next_id() {
        Ok(id) => {
            let decomposed_id = decompose(id);
            Ok((
                id as i64,
                // time is in a non standard 1/100 second unix epoch time format used by sonyflake.
                decomposed_id.time as i64,
                decomposed_id.machine_id as u16,
            ))
        }
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

/// Gets integer unix timestamp in the non standard 1/100 second unix epoch time format used in our IDs.
pub fn _get_currentrtf_timestamp() -> i64 {
    Utc::now().timestamp_millis() / 10
}

/// Gets chronos DateTime from a 1/100 second unix epoch timestamp.
pub fn _new_date_time_from_timestamp(timestamp: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp_millis(timestamp * 10)
}

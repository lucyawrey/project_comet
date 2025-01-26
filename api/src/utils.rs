use sonyflake::{decompose, Sonyflake};
use tonic::Status;

pub fn next_id(sf: &Sonyflake) -> Result<(i64, i64), Status> {
    match sf.next_id() {
        Ok(id) => Ok((id as i64, decompose(id).time as i64)),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

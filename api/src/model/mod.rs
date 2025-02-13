pub mod fields;
pub mod tables;

pub enum Ref {
    Id(i64),
    Name(String),
}

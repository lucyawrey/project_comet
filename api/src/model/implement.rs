use super::fields::AssetData;
use sqlx::{
    encode::IsNull, sqlite::SqliteValueRef, Decode, Encode, Sqlite, Type, TypeInfo, ValueRef,
};
use std::{error::Error, fmt};

impl<'r> Decode<'r, Sqlite> for AssetData {
    fn decode(
        value: SqliteValueRef<'r>,
    ) -> Result<AssetData, Box<(dyn Error + Send + Sync + 'static)>> {
        match value.type_info().name() {
            "BLOB" => Ok(AssetData::Blob(
                (<Vec<u8> as Decode<Sqlite>>::decode(value))?,
            )),
            "TEXT" => Ok(AssetData::String(
                (<String as Decode<Sqlite>>::decode(value))?,
            )),
            _ => Err("Database column is an unsupported type.".into()),
        }
    }
}

impl<'r> Encode<'r, Sqlite> for AssetData {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        match &self {
            AssetData::Blob(b) => <Vec<u8> as Encode<Sqlite>>::encode_by_ref(b, buf),
            AssetData::String(s) => <String as Encode<Sqlite>>::encode_by_ref(s, buf),
        }
    }

    fn produces(&self) -> Option<<Sqlite as sqlx::Database>::TypeInfo> {
        match &self {
            AssetData::Blob(_) => Some(<Vec<u8> as Type<Sqlite>>::type_info()),
            AssetData::String(_) => Some(<String as Type<Sqlite>>::type_info()),
        }
    }
}

impl fmt::Display for AssetData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Type<Sqlite> for AssetData {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl TypeInfo for AssetData {
    fn is_null(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        match &self {
            AssetData::Blob(_) => "BLOB",
            AssetData::String(_) => "TEXT",
        }
    }
}

use async_graphql::{InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDecimal(pub Decimal);

#[Scalar(name = "Decimal")]
impl ScalarType for CustomDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        let dec = value.to_string().parse::<Decimal>().unwrap();
        Ok(CustomDecimal(dec))
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

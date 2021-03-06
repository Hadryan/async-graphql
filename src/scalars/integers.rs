use crate::{InputValueError, InputValueResult, Result, ScalarType, Value};
use async_graphql_derive::Scalar;

macro_rules! impl_integer_scalars {
    ($($ty:ty),*) => {
        $(
        #[Scalar(internal)]
        impl ScalarType for $ty {
            fn type_name() -> &'static str {
                "Int"
            }

            fn description() -> Option<&'static str> {
                Some("The `Int` scalar type represents non-fractional signed whole numeric values. Int can represent values between -(2^31) and 2^31 - 1.")
            }

            fn parse(value: &Value) -> InputValueResult<Self> {
                match value {
                    Value::Int(n) => Ok(*n as Self),
                    _ => Err(InputValueError::ExpectedType)
                }
            }

            fn is_valid(value: &Value) -> bool {
                match value {
                    Value::Int(_) => true,
                    _ => false
                }
            }

            fn to_json(&self) -> Result<serde_json::Value> {
                Ok((*self).into())
            }
        }
        )*
    };
}

impl_integer_scalars!(i8, i16, i32, u8, u16, u32);

macro_rules! impl_int64_scalars {
    ($($ty:ty),*) => {
        $(
        #[Scalar(internal)]
        impl ScalarType for $ty {
            fn type_name() -> &'static str {
                "Int64"
            }

            fn description() -> Option<&'static str> {
                Some("The `Int64` scalar type represents non-fractional signed whole numeric values. Int can represent values between -(2^64) and 2^64 - 1.")
            }

            fn parse(value: &Value) -> InputValueResult<Self> {
                match value {
                    Value::Int(n) => Ok(*n as Self),
                    Value::String(s) => Ok(s.parse()?),
                    _ => Err(InputValueError::ExpectedType)
                }
            }

            fn is_valid(value: &Value) -> bool {
                match value {
                    Value::Int(_) | Value::String(_) => true,
                    _ => false
                }
            }

            fn to_json(&self) -> Result<serde_json::Value> {
                Ok(self.to_string().into())
            }
        }
        )*
    };
}

impl_int64_scalars!(i64, u64);

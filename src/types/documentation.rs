use serde::{Deserialize, Serialize};
use crate::types::doc_type::FuncType;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Doc {
    FuncType(FuncType),
    ErrorType(crate::types::error_type::ErrorType),
    NoneType(crate::types::none_type::NoneType),
}

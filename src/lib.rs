pub mod generate;
pub mod types;

pub use crate::generate::generate_tsx as gtsx;
pub use crate::types::{
    class_type,
    documentation,
    doc_type,
    error_type,
    none_type,
};

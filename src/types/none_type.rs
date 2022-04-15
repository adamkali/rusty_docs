#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct NoneType {
    pub name: String,
    pub value: Option<String>,
}

impl NoneType {
    pub fn new() -> NoneType {
        NoneType {
            name: "none".to_string(),
            value: None,
        }
    }
}

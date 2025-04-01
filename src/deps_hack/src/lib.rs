pub use serde;
pub use serde_json;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MyStruct {
    pub field: Option<bool>
}
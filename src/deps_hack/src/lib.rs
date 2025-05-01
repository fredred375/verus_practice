pub use serde;
pub use serde_json;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MyStruct {
    pub field: Option<Vec<MyBool>>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MyBool {
    pub b: i32
}
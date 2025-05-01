pub use serde;
pub use serde_json;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MyStruct {
    pub field: Option<Vec<MyNum>>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MyNum {
    pub num: i32
}
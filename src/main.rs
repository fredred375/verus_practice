use std::vec;

use deps_hack::MyStruct;
use deps_hack::serde_json;
#[path = "mystruct/mod.rs"]
pub mod mystruct;
use crate::mystruct::exec::check::check_mystruct;
use crate::mystruct::trusted::exec_types::MyStruct as TrustedMyStruct;

fn main() {
    let json = serde_json::json!({
        "field": vec![serde_json::json!({"num": 1}), serde_json::json!({"num": 0})]
    });
    let mystruct = serde_json::from_value::<MyStruct>(json).unwrap();
    println!("{:?}", mystruct);
    let res = check_mystruct(&TrustedMyStruct::from_hack(mystruct));
    println!("Check result: {}", res);
}

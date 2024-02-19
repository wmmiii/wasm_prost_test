use prost::Message;
use proto_lib::wasm_prost_test::TestProto;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

pub fn main() {
    println!("Main");
}

#[wasm_bindgen]
pub fn round_trip() -> String {
    let definition = TestProto {
        name: "foo".to_string(),
    };
    let encoded = definition.encode_to_vec();

    let decoded = TestProto::decode(&mut Cursor::new(encoded)).unwrap();

    return decoded.name;
}

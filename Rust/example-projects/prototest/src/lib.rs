pub mod prototest {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/prototest.items.rs"));
    }
}

use prost::Message;
use prototest::items;

pub fn test_proto() {
    let data = create_data();
    let encoded_data = serialize_data(&data);
    let decoded_data = deserialize_data(&encoded_data);
    // assert data == decoded_data
    assert_eq!(data, decoded_data);
    println!("{:#?}", data);
    println!("{:#?}", decoded_data);
}

fn create_data() -> items::Data {
    let mut data = items::Data::default();
    data.color = "blue".to_string();
    data.size = items::data::Size::Large as i32;
    data.repeated_data.push("Hello".to_string());
    data.repeated_data.push("World".to_string());
    data
}

fn serialize_data(data: &items::Data) -> Vec<u8> {
    let mut buf = vec![];
    data.encode(&mut buf).unwrap();
    buf
}

fn deserialize_data(encoded_data: &Vec<u8>) -> items::Data {
    items::Data::decode(&encoded_data[..]).unwrap()
}

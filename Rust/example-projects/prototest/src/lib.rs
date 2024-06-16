// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod prototest {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/prototest.items.rs"));
    }
}

use prototest::items;

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    // shirt.set_size(items::shirt::Size::Large);
    shirt.size = items::shirt::Size::Large as i32;
    shirt
}

pub fn test_proto() {
    let shirt = create_large_shirt("blue".to_string());
    assert_eq!(shirt.color, "blue");
    assert_eq!(shirt.size, items::shirt::Size::Large as i32);
    println!("{:#?}", shirt);
}

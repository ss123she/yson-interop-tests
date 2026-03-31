use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use yson_rs::{YsonFormat, attributes::WithAttributes, from_slice, to_vec};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Meta {
    description: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ComprehensiveData {
    int_min: i64,
    int_max: i64,
    uint_max: u64,
    int_zero: i64,
    int_neg_one: i64,

    float_nan: f64,
    float_inf: f64,
    float_neg_inf: f64,
    float_zero: f64,

    empty_str: String,
    special_str: String,
    #[serde(with = "serde_bytes")]
    byte_array: Vec<u8>,

    some_val: Option<String>,
    none_val: Option<String>,

    nested_list: Vec<Vec<i32>>,
    empty_map: BTreeMap<String, i32>,

    #[serde(rename = "attributed_str")]
    attributed_str: WithAttributes<String, Meta>,

    #[serde(rename = "attributed_list")]
    attributed_list: AttributedList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AttributedList {
    #[serde(rename = "@list_id")]
    list_id: String,
    #[serde(rename = "$value")]
    items: Vec<f64>,
}

fn create_initial_data() -> ComprehensiveData {
    ComprehensiveData {
        int_min: i64::MIN,
        int_max: i64::MAX,
        uint_max: u64::MAX,
        int_zero: 0,
        int_neg_one: -1,

        float_nan: std::f64::NAN,
        float_inf: std::f64::INFINITY,
        float_neg_inf: std::f64::NEG_INFINITY,
        float_zero: 0.0,

        empty_str: "".to_string(),
        special_str: "Line1\nLine2\t\0\"\\".to_string(),
        byte_array: vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF],

        some_val: Some("Present".to_string()),
        none_val: None,

        nested_list: vec![vec![], vec![1, 2, 3], vec![-100]],
        empty_map: BTreeMap::new(),

        attributed_str: WithAttributes {
            attributes: Meta {
                description: "Just a string".into(),
                timestamp: 123456789,
            },
            value: "Hello with attributes".into(),
        },
        attributed_list: AttributedList {
            list_id: "list-x".into(),
            items: vec![1.1, 2.2],
        },
    }
}

fn verify_modified_data(ds: &ComprehensiveData, format_name: &str) {
    assert_eq!(
        ds.int_max,
        i64::MAX - 1,
        "[{}] Go should have modified int_max",
        format_name
    );
    assert_eq!(
        ds.uint_max,
        u64::MAX - 1,
        "[{}] Go should have modified uint_max",
        format_name
    );
    assert!(ds.float_nan.is_nan(), "[{}] NaN is broken", format_name);
    assert!(
        ds.float_inf.is_infinite() && ds.float_inf.is_sign_positive(),
        "[{}] Inf is broken",
        format_name
    );

    assert_eq!(
        ds.special_str, "Line1\nLine2\t\0\"\\_modified",
        "[{}] Special string broken",
        format_name
    );
    assert_eq!(
        ds.byte_array,
        vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF, 0x42],
        "[{}] Byte array broken",
        format_name
    );

    assert_eq!(ds.none_val, None, "[{}] None value broken", format_name);
    assert_eq!(
        ds.some_val.as_deref(),
        Some("Present_modified"),
        "[{}] Option broken",
        format_name
    );

    assert_eq!(
        ds.nested_list[1],
        vec![1, 2, 3, 4],
        "[{}] Nested list append broken",
        format_name
    );

    assert_eq!(
        ds.attributed_str.attributes.timestamp, 999999,
        "[{}] Attributes modification broken",
        format_name
    );
    assert_eq!(
        ds.attributed_str.value, "Hello with attributes_from_go",
        "[{}] Attributed value broken",
        format_name
    );
}

fn main() {
    let bin_to_go = "../data/rust_to_go_binary.bin";
    let bin_from_go = "../data/go_to_rust_binary.bin";
    let txt_to_go = "../data/rust_to_go_text.txt";
    let txt_from_go = "../data/go_to_rust_text.txt";

    let rust_data = create_initial_data();
    fs::create_dir_all("../data").ok();

    let bin_data = to_vec(&rust_data, YsonFormat::Binary).expect("Binary serialization failed");
    fs::write(bin_to_go, bin_data).expect("Write failed");

    let txt_data = to_vec(&rust_data, YsonFormat::Text).expect("Text serialization failed");
    fs::write(txt_to_go, txt_data).expect("Write failed");

    println!("Rust: [OK] Wrote Binary and Text datasets for Go");

    if std::path::Path::new(bin_from_go).exists() && std::path::Path::new(txt_from_go).exists() {
        let input_bin = fs::read(bin_from_go).unwrap();
        let ds_bin: ComprehensiveData = from_slice(&input_bin, YsonFormat::Binary).unwrap();
        verify_modified_data(&ds_bin, "Binary");

        let input_txt = fs::read(txt_from_go).unwrap();
        let ds_txt: ComprehensiveData = from_slice(&input_txt, YsonFormat::Text).unwrap();
        verify_modified_data(&ds_txt, "Text");

        println!("Rust: [OK] Successfully read and verified Go's datasets!");
        println!("Rust: FULL COMPATIBILITY CONFIRMED (BINARY + TEXT)");
    } else {
        println!("Rust: Waiting for Go script to run...");
    }
}

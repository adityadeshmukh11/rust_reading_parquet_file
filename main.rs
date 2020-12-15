// use parquet::file::reader::{FileReader, SerializedFileReader};
// use std::{fs::File, path::Path};

// let path = Path::new("/home/aditya/Desktop/test-rust/new_parq/parquet/username1.parquet");
// if let Ok(file) = File::open(&path) {
//     let file = File::open(&path).unwrap();
//     let reader = SerializedFileReader::new(file).unwrap();

//     let parquet_metadata = reader.metadata();
//     assert_eq!(parquet_metadata.num_row_groups(), 1);

//     let row_group_reader = reader.get_row_group(0).unwrap();
//     assert_eq!(row_group_reader.num_columns(), 1);
// }
fn main() {
    let the_file = r#"{
        "Name": "Aditya Deshmukh",
        "Age": 21,
        "Address": [
            {"Street": "Mg Road",
            "City": "Mumbai",
            "Country": "India",
            "Country_Code":91}
        ],
        "Userdata": [
            "First_Name":"aditya",
            "Last_Name":"deshmukh"
        ]
    }"#;

    let json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
}
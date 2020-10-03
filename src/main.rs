/* single parquet file reading from documentation*/
use parquet::file::reader::SerializedFileReader;
use std::convert::TryFrom;

fn main()
{let paths = vec![
    "./parquet/userdata1.parquet"
];
// // Create a reader for each file and flat map rows
let rows = paths.iter()
    .map(|p| SerializedFileReader::try_from(*p).unwrap())
    .flat_map(|r| r.into_iter());

for row in rows {
    println!("{}", row);
    // println!("1");
}
}

/* single parquet file reading*/
// use std::fs::File;
// use std::path::Path;
// use parquet::file::reader::{FileReader, SerializedFileReader};

// fn main()
// {let file = File::open(&Path::new("./parquet/userdata1.parquet")).unwrap();
// let reader = SerializedFileReader::new(file).unwrap();
// let mut iter = reader.get_row_iter(None).unwrap();
// while let Some(record) = iter.next() {
//   println!("{}", record);
// }
// }

// ****************//read from json file// ****************//
// use rustc_serialize::json::Json;
// use std::fs::File;
// use std::io::copy;
// use std::io::stdout;

// fn main() {
//     let mut file = File::open("/home/aditya/Desktop/final_wala/hello_cargo/text.json").unwrap();
//     let mut stdout = stdout();
//     let str = &copy(&mut file, &mut stdout).unwrap().to_string();
//     let _data = Json::from_str(str).unwrap();
// }


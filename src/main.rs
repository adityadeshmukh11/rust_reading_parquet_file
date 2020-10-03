/* single parquet file reading from documentation*/
// use parquet::file::reader::SerializedFileReader;
// use std::convert::TryFrom;

// fn main()
// {let paths = vec![
//     "/home/aditya/Desktop/parquet/userdata1.parquet"
// ];
// // Create a reader for each file and flat map rows
// let rows = paths.iter()
//     .map(|p| SerializedFileReader::try_from(*p).unwrap())
//     .flat_map(|r| r.into_iter());

// for row in rows {
//     println!("{}", row);
//     // println!("1");
// }
// }

/* single parquet file reading*/
// use std::fs::File;
// use std::path::Path;
// use parquet::file::reader::{FileReader, SerializedFileReader};

// fn main()
// {let file = File::open(&Path::new("/home/aditya/Desktop/parquet/userdata1.parquet")).unwrap();
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

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
    }

fn main()
{
typed_example();
}
/* single parquet file reading from documentation*/
// use parquet::file::reader::SerializedFileReader;
// use std::convert::TryFrom;

// fn main()
// {let paths = vec![
//     "./parquet/userdata1.parquet"
// ];
// Create a reader for each file and flat map rows
// let rows = paths.iter()
//     .map(|p| SerializedFileReader::try_from(*p).unwrap())
//     .flat_map(|r| r.into_iter());

// for row in rows {
//     println!("{}", row);
//     // println!("1");
// }
// }

/* single parquet file reading and writing
if sample.parquet not exists then it creates a parquet file and writes code from another .parquet file
(in the form of json).
if sample.parquet exists then it overrides same file (in the form of json).
***********************************************main code***********************************************
*/

use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::{
    file::{
        properties::WriterProperties,
        writer::{FileWriter, SerializedFileWriter},
    },
    schema::parser::parse_message_type,
};
use std::fs::File;
use std::path::Path;
use std::{fs, rc::Rc};

fn reader_parquet()
{let file = File::open(&Path::new("./parquet/userdata1.parquet")).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        println!("{}", record);
    }
}

fn writer_parquet()
{
    let path = Path::new("/home/aditya/Desktop/final_wala/hello_cargo/parquet/sample.parquet");
    let message_type = "
  message schema {
    REQUIRED INT32 b;
  }
";
    // let mut new=0;
    let schema = Rc::new(parse_message_type(message_type).unwrap());
    let props = Rc::new(WriterProperties::builder().build());
    let file = fs::File::create(&path).unwrap();
    let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
    let mut row_group_writer = writer.next_row_group().unwrap();
    while let Some(col_writer) = row_group_writer.next_column().unwrap() {
        // ... write values to a column writer
        row_group_writer.close_column(col_writer).unwrap();
        // new=new+1;
    }
    writer.close_row_group(row_group_writer).unwrap();
    writer.close().unwrap();

    let bytes = fs::read(&path).unwrap();
    assert_eq!(&bytes[0..4], &[b'P', b'A', b'R', b'1']);
    // println!("{}",new);
}

fn main() {
    reader_parquet();
    writer_parquet();
}

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

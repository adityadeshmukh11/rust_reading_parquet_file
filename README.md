# rust_reading_parquet_file
Implementation of reading and writing content to .parquet file in rust.

cargo install parquet       (package==`parquet v2.0.0`)(make sure you must add the parquet in the environment path in order to use parquet-schema,parquet-read,parquet-rowcount)                                                                                                                                               
delete file1.parquet (here we are generating file1.parquet by writing some content)                                                                       
cargo run       (it will generate file1.parquet and then perform following commands)                                                                      
parquet-schema file1.parquet true/false (it will show you the schema)                                                                                         
parquet-read file1.parquet 5(here 5 is the row_number & parquet-read will read upto 5 rows)


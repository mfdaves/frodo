use frodo::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let vql_time = Vql::try_from(0x7899).unwrap();
    let bytes = vql_time.encode_bytes();
    println!("{:?}", bytes);
}

//TODO:
//-> .mid file

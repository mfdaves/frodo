use frodo::*;
use std::fs::File; 
use std::io::Read;



fn main(){
    let mut file = File::open("/home/mfdaves/Downloads/Zelda.mid").unwrap();

    let mut buffer = vec![0u8;14];

    println!("{:?}", &file);

    let foo = file.read_exact(&mut buffer);

    println!("{:?}", &buffer);

    let header = Header::from_bytes(&buffer);

    println!("{:?}", header);
}




//TODO: 
//-> .mid file 
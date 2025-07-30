use frodo::MidiMessage;

fn main() {
    let status_byte = 0x92;
    let msg = MidiMessage::from_status_byte(status_byte).unwrap();
    println!("{:?}", msg);
}




//TODO: 
//-> .mid file 
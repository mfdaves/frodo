use crate::chunktype::ChunkType; 
use crate::error::MidiError;
use std::convert::TryFrom;

#[derive(Debug,Clone,Copy)]
pub struct Track{
	chunk_type:ChunkType,
	length:u32,
	track_event:TrackEvent
}

impl Track{

}


#[derive(Debug,Clone,Copy)]
pub struct TrackEvent{
	v_time:Vql, 
	//devo mettere questi tipi di eventi: track_event = <v_time> + <midi_event> | <meta_event> | <sysex_event>
}


#[derive(Debug,Clone,Copy)]
pub struct Vql(u32); 


impl Vql{
	pub const MAX: u32 = 0x0FFF_FFFF;

	pub fn value(&self)->u32{
		self.0
	}

	pub fn encode(&self) -> [u8;4]{
		let mut buffer = [0u8; 4];
		let mut value = self.0;

		let mut i = 4;

		loop{
			i -= 1; 
			buffer[i] = (value & 0x7F) as u8; 
			value >>= 7; 

			if value == 0 {
				break;
			}
		}

		for j in i..3{
			//set MSB 1
			buffer[j] |= 0x80;
		}

		buffer
	}

	pub fn encode_len(&self)->usize{
		match self.0{
			0..=0x7F => 1, 
			0x80..=0x3FFF => 2, 
			0x4000..=0x1FFFFF => 3, 
			_ => 4
		}
	}

	pub fn encode_bytes(&self)->Vec<u8>{
		let full = self.encode();
		full[(4-self.encode_len())..].to_vec()
	}
}


impl TryFrom<u32> for Vql{
	type Error = MidiError;

	fn try_from(value:u32)->Result<Self,Self::Error>{
		if value <= Vql::MAX {
			Ok(Self(value))
		} else {
			Err(MidiError::InvalidVqlInput(value))
		}
	}
}



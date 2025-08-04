#[derive(Debug, Clone, Copy)]
pub enum ChunkType{
	Header,
	Track
}

impl ChunkType{
	pub fn as_bytes(&self)->[u8;4]{
		match self{
			ChunkType::Header => *b"MThd",
			ChunkType::Track => *b"MTrk"
		}
	}
}
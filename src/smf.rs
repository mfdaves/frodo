use crate::header::Header;
use crate::track::Track;
use crate::TrackEvent;

// ** SMF ** Standard Midi File
#[derive(Debug, Clone)]
pub struct Smf {
    header: Header,
    tracks: Vec<Track>,
}
impl Smf {
    pub fn new(header: Header, tracks: Vec<Track>) -> Self {
        Self { header, tracks }
    }

    pub fn from_slice(header: Header, tracks: &[Track]) -> Self {
        Self {
            header,
            tracks: tracks.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        //14 is the fixed header bytes len
        let mut bytes: Vec<u8> =
            Vec::with_capacity(14 + self.tracks.iter().map(|x| x.length()).sum::<usize>());
        bytes.extend_from_slice(&self.header.to_bytes());
        self.tracks
            .iter()
            .for_each(|track| bytes.extend_from_slice(&track.to_bytes()));

        bytes
    }
}

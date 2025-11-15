A simple MIDI 1.0 encoder / decoder written in Rust.


My intention was to build a simple lib allow me to create music easily.
Things that are missing:
  - deconding from midi file
  - json2midi
  - running status (doesnt repeat status for chain of event sharing the same status byte, I'll work on it :) ) 
  - map all programs to an enumeration (so convert it easily from string to 7 bit representation, I refer to the json2midi tool) 

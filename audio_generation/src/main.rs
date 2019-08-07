use std::io;
use std::io::prelude::*;
use std::fs::File;

const u8 CHUNK_ID_SIZE = 4;
const u8 CHUNK_SIZE_SIZE = 4;
const u8 FORMAT_SIZE = 4;
const u8 SUB_CHUNK_1_ID_SIZE = 4;
const u8 SUB_CHUNK_1_SIZE_SIZE = 4;
const u8 AUDIO_FORMAT_SIZE = 2;
const u8 NUM_CHANNEL_SIZE = 2;
const u8 SAMPLE_RATE_SIZE = 4;
const u8 BYTE_RATE_SIZE = 4;
const u8 BLOCK_ALIGN_SIZE = 2;
const u8 BITS_PER_SAMPLE_SIZE = 2;
const u8 SUB_CHUNK_2_ID_SIZE = 4;
const u8 SUB_CHUNK_2_SIZE_SIZE = 4;

const u32 CHUNK_ID_LOC = 0;
const u32 CHUNK_SIZE_LOC = 4;
const u32 FORMAT_LOC = 8;
const u32 SUB_CHUNK_1_ID_LOC = 12;
const u32 SUB_CHUNK_1_SIZE_LOC = 16;
const u32 AUDIO_FORMAT_LOC = 20;
const u32 NUM_CHANNEL_LOC = 22;
const u32 SAMPLE_RATE_LOC = 24;
const u32 BYTE_RATE_LOC = 28;
const u32 BLOCK_ALIGN_LOC = 32;
const u32 BITS_PER_SAMPLE_LOC = 34;
const u32 SUB_CHUNK_2_ID_LOC = 36;
const u32 SUB_CHUNK_2_SIZE_LOC = 40;

#[derive(Clone, Copy)]
struct Wav {
    pub ChunkID : u8,
    pub ChunkSize : u8,
    pub Format : u8,
    pub Subchunk1ID : u8,
    pub Subchunk1Size : u8,
    pub AudioFormat : u8,
    pub NumChannels : u8,
    pub SampleRate : u8,
    pub ByteRate : u8,
    pub BlockAlign : u8,
    pub BitsPerSample : u8,
    pub Subchunk2ID : u8,
    pub Subchunk2Size : u8,
}

// 44 Bytes of WAV header information, ignoring extra parameters


fn read_wav(String file_name) -> Vec<u8> {
   let mut f = File::open(file_name)?;
   let mut buffer = Vec::new();

   f.read_to_end(&mut buffer)?;
   return buffer;
}

fn construct_merged_header(wav_a : &Vec<u8>, wav_b : &Vec<u8>) {
}

fn main() {
    let sample_one_data : Vec<u8> = read_wav("sample_1.wav");    
    let sample_two_data : Vec<u8> = read_wav("sample_2.wav");
}

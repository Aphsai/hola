use std::io;
use std::io::prelude::*;
use std::fs::File;

const CHUNK_ID_SIZE : u8 = 4;
const CHUNK_SIZE_SIZE : u8 = 4;
const FORMAT_SIZE : u8 = 4;
const SUBCHUNK_1_ID_SIZE : u8 = 4;
const SUBCHUNK_1_SIZE_SIZE : u8 = 4;
const AUDIO_FORMAT_SIZE : u8 = 2;
const NUM_CHANNEL_SIZE : u8 = 2;
const SAMPLE_RATE_SIZE : u8 = 4;
const BYTE_RATE_SIZE : u8 = 4;
const BLOCK_ALIGN_SIZE : u8 = 2;
const BITS_PER_SAMPLE_SIZE : u8 = 2;
const SUBCHUNK_2_ID_SIZE : u8 = 4;
const SUBCHUNK_2_SIZE_SIZE : u8 = 4;

const CHUNK_ID_LOC : u32 = 0;
const CHUNK_SIZE_LOC : u32 = 4;
const FORMAT_LOC : u32 = 8;
const SUBCHUNK_1_ID_LOC : u32 = 12;
const SUBCHUNK_1_SIZE_LOC : u32 = 16;
const AUDIO_FORMAT_LOC : u32 = 20;
const NUM_CHANNEL_LOC : u32 = 22;
const SAMPLE_RATE_LOC : u32 = 24;
const BYTE_RATE_LOC : u32 = 28;
const BLOCK_ALIGN_LOC : u32 = 32;
const BITS_PER_SAMPLE_LOC : u32 = 34;
const SUBCHUNK_2_ID_LOC : u32 = 36;
const SUBCHUNK_2_SIZE_LOC : u32 = 40;
const DATA_LOC : u32 = 44;

struct WAV {
    pub chunk_id : u32,
    pub chunk_size : u32,
    pub format : u32,
    pub subchunk_1_id : u32,
    pub subchunk_1_size : u32,
    pub audio_format : u32,
    pub num_channels : u32,
    pub sample_rate : u32,
    pub byte_rate : u32,
    pub block_align : u32,
    pub bits_per_sample : u32,
    pub subchunk_2_id : u32,
    pub subchunk_2_size : u32,
    pub data : Vec<u8>,
}

fn convert_bytes_to_int(buf : &Vec<u8>, loc : u32, size: u8, is_big_endian: bool) -> u32 {
    let loc = loc as usize;
    let size = size as usize;
    if is_big_endian { buf[loc.. loc + size].iter().fold(0, |converted_integer, &x| converted_integer << 8 | x as u32) }
    else { buf[loc.. loc + size].iter().rev().fold(0, |converted_integer, &x| converted_integer << 8 | x as u32) }
}

fn read_wav(file_name : String) -> WAV {
   let mut buffer = Vec::new();
   let mut f = File::open(file_name).unwrap();
   f.read_to_end(&mut buffer).unwrap();

   return WAV {
    chunk_id : convert_bytes_to_int(&buffer, CHUNK_ID_LOC, CHUNK_ID_SIZE, true),
    chunk_size : convert_bytes_to_int(&buffer, CHUNK_SIZE_LOC, CHUNK_SIZE_SIZE, false), 
    format : convert_bytes_to_int(&buffer, FORMAT_LOC, FORMAT_SIZE, true), 
    subchunk_1_id : convert_bytes_to_int(&buffer, SUBCHUNK_1_ID_LOC, SUBCHUNK_1_ID_SIZE, true),
    subchunk_1_size : convert_bytes_to_int(&buffer, SUBCHUNK_1_SIZE_LOC, SUBCHUNK_1_SIZE_SIZE, true),
    audio_format :  convert_bytes_to_int(&buffer, AUDIO_FORMAT_LOC, AUDIO_FORMAT_SIZE, false),
    num_channels : convert_bytes_to_int(&buffer, NUM_CHANNEL_LOC, NUM_CHANNEL_SIZE, false),
    sample_rate : convert_bytes_to_int(&buffer, SAMPLE_RATE_LOC, SAMPLE_RATE_SIZE, false),
    byte_rate : convert_bytes_to_int(&buffer, BYTE_RATE_LOC, BYTE_RATE_SIZE, false),
    block_align : convert_bytes_to_int(&buffer, BLOCK_ALIGN_LOC, BLOCK_ALIGN_SIZE, false),
    bits_per_sample : convert_bytes_to_int(&buffer, BITS_PER_SAMPLE_LOC, BITS_PER_SAMPLE_SIZE, true),
    subchunk_2_id : convert_bytes_to_int(&buffer, SUBCHUNK_2_ID_LOC, SUBCHUNK_2_ID_SIZE, false),
    subchunk_2_size : convert_bytes_to_int(&buffer, SUBCHUNK_2_SIZE_LOC, SUBCHUNK_2_SIZE_SIZE, false),
    data : buffer[DATA_LOC as usize ..].to_vec(),
   }
}

fn construct_merged_header(wav_a : &Vec<u8>, wav_b : &Vec<u8>) {
}

fn main() {
    let sample_one_data : WAV = read_wav("sample_1.wav".to_string());    
    let sample_two_data : WAV = read_wav("sample_2.wav".to_string());
    let merged_data : WAV =  
}

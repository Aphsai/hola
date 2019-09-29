#![allow(dead_code)]
extern crate byteorder;

use std::io::{ BufWriter };
use std::io::prelude::*;
use std::fs::File;
use byteorder::{ BigEndian, LittleEndian, WriteBytesExt, ByteOrder };

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

pub struct Wave {
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
    if is_big_endian { BigEndian::read_uint(&buf[loc.. loc + size], size) as u32 }
    else { LittleEndian::read_uint(&buf[loc.. loc + size], size) as u32 }
}

impl Wave {

    pub fn read_wav(file_name : &str) -> Wave {

        let mut buffer = Vec::new();
        let mut f = File::open(file_name).unwrap();
        f.read_to_end(&mut buffer).unwrap();
    
        Wave {

            chunk_id : convert_bytes_to_int(&buffer, CHUNK_ID_LOC, CHUNK_ID_SIZE, true),
            chunk_size : convert_bytes_to_int(&buffer, CHUNK_SIZE_LOC, CHUNK_SIZE_SIZE, false), 
            format : convert_bytes_to_int(&buffer, FORMAT_LOC, FORMAT_SIZE, true), 
            subchunk_1_id : convert_bytes_to_int(&buffer, SUBCHUNK_1_ID_LOC, SUBCHUNK_1_ID_SIZE, true),
            subchunk_1_size : convert_bytes_to_int(&buffer, SUBCHUNK_1_SIZE_LOC, SUBCHUNK_1_SIZE_SIZE, false),
            audio_format :  convert_bytes_to_int(&buffer, AUDIO_FORMAT_LOC, AUDIO_FORMAT_SIZE, false),
            num_channels : convert_bytes_to_int(&buffer, NUM_CHANNEL_LOC, NUM_CHANNEL_SIZE, false),
            sample_rate : convert_bytes_to_int(&buffer, SAMPLE_RATE_LOC, SAMPLE_RATE_SIZE, false),
            byte_rate : convert_bytes_to_int(&buffer, BYTE_RATE_LOC, BYTE_RATE_SIZE, false),
            block_align : convert_bytes_to_int(&buffer, BLOCK_ALIGN_LOC, BLOCK_ALIGN_SIZE, false),
            bits_per_sample : convert_bytes_to_int(&buffer, BITS_PER_SAMPLE_LOC, BITS_PER_SAMPLE_SIZE, false),
            subchunk_2_id : convert_bytes_to_int(&buffer, SUBCHUNK_2_ID_LOC, SUBCHUNK_2_ID_SIZE, true),
            subchunk_2_size : convert_bytes_to_int(&buffer, SUBCHUNK_2_SIZE_LOC, SUBCHUNK_2_SIZE_SIZE, false),
            data : buffer[DATA_LOC as usize ..].to_vec(),

        }
    }
    
    pub fn append(&mut self, wav_b : &mut Wave) {
        self.chunk_size += wav_b.subchunk_2_size;
        self.subchunk_2_size += wav_b.subchunk_2_size;
        self.data.append(&mut wav_b.data);
    }
    
    pub fn write_to_file(&self, filename : &str) {
    
        let file = File::create(filename).expect("Unable to create file...");
        let mut file = BufWriter::new(file);
    
        file.write_u32::<BigEndian>(self.chunk_id).unwrap();
        file.write_u32::<LittleEndian>(self.chunk_size).unwrap();
        file.write_u32::<BigEndian>(self.format).unwrap();
        file.write_u32::<BigEndian>(self.subchunk_1_id).unwrap();
        file.write_u32::<LittleEndian>(self.subchunk_1_size).unwrap();
        file.write_u16::<LittleEndian>(self.audio_format as u16).unwrap();
        file.write_u16::<LittleEndian>(self.num_channels as u16).unwrap();
        file.write_u32::<LittleEndian>(self.sample_rate).unwrap();
        file.write_u32::<LittleEndian>(self.byte_rate).unwrap();
        file.write_u16::<LittleEndian>(self.block_align as u16).unwrap();
        file.write_u16::<LittleEndian>(self.bits_per_sample as u16).unwrap();
        file.write_u32::<BigEndian>(self.subchunk_2_id).unwrap();
        file.write_u32::<LittleEndian>(self.subchunk_2_size).unwrap();
    
        for byte in &self.data {
            file.write_u8(*byte).unwrap();
        }

    }

}


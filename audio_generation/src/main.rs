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

struct Wave {

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
    //if is_big_endian { buf[loc.. loc + size].iter().fold(0, |converted_integer, &x| converted_integer << 8 | x as u32) }
    //else { buf[loc.. loc + size].iter().rev().fold(0, |converted_integer, &x| converted_integer << 8 | x as u32) }
}

fn read_wav(file_name : &str) -> Wave {
    let mut buffer = Vec::new();
    let mut f = File::open(file_name).unwrap();
    f.read_to_end(&mut buffer).unwrap();

    return Wave {
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

fn construct_merged_wav(wav_a : &mut Wave, wav_b : &mut Wave) -> Wave {

    wav_a.data.append(&mut wav_b.data);

    return Wave {
        chunk_id : wav_a.chunk_id,
        chunk_size : wav_a.chunk_size + wav_b.subchunk_2_size,
        format : wav_a.format,
        subchunk_1_id : wav_a.subchunk_1_id,
        subchunk_1_size : wav_a.subchunk_1_size,
        audio_format : wav_a.audio_format,
        num_channels : wav_a.num_channels,
        sample_rate : wav_a.sample_rate,
        byte_rate : wav_a.byte_rate,
        block_align : wav_a.block_align,
        bits_per_sample : wav_a.bits_per_sample,
        subchunk_2_id : wav_a.subchunk_2_id,
        subchunk_2_size : wav_a.subchunk_2_size + wav_b.subchunk_2_size,
        data : wav_a.data.clone(),
    };

}

fn write_wav_to_file(wav: &Wave, filename : &str) {

    let file = File::create(filename).expect("Unable to create file...");
    let mut file = BufWriter::new(file);

    file.write_u32::<BigEndian>(wav.chunk_id).unwrap();
    file.write_u32::<LittleEndian>(wav.chunk_size).unwrap();
    file.write_u32::<BigEndian>(wav.format).unwrap();
    file.write_u32::<BigEndian>(wav.subchunk_1_id).unwrap();
    file.write_u32::<LittleEndian>(wav.subchunk_1_size).unwrap();
    file.write_u16::<LittleEndian>(wav.audio_format as u16).unwrap();
    file.write_u16::<LittleEndian>(wav.num_channels as u16).unwrap();
    file.write_u32::<LittleEndian>(wav.sample_rate).unwrap();
    file.write_u32::<LittleEndian>(wav.byte_rate).unwrap();
    file.write_u16::<LittleEndian>(wav.block_align as u16).unwrap();
    file.write_u16::<LittleEndian>(wav.bits_per_sample as u16).unwrap();
    file.write_u32::<BigEndian>(wav.subchunk_2_id).unwrap();
    file.write_u32::<LittleEndian>(wav.subchunk_2_size).unwrap();

    for byte in &wav.data {
        file.write_u8(*byte).unwrap();
    }
}

fn main() {
    let mut sample_one_data : Wave = read_wav("sample_1.wav");    
    let mut sample_two_data : Wave = read_wav("sample_2.wav");
    let merged_wav : Wave =  construct_merged_wav(&mut sample_one_data, &mut sample_two_data);
    write_wav_to_file(&merged_wav, "merged.wav");
}

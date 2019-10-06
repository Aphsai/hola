
mod wave;

use crate::wave::*;
use std::process::Command;
use std::str;
use std::env;

const PROJECT_ROOT : &str = "../../../";
const AUDIO_FILE_PATH : &str = "audio_files";
const RNN_PATH : &str = "rnn";
const PREDICTOR : &str = "predict.py";

fn set_relative_path(file_path : &str) {
    let mut current_directory = env::current_exe().unwrap();
    current_directory.pop();
    current_directory.push(file_path);
    env::set_current_dir(current_directory.as_path()).unwrap();
}

fn predict(word : &str) -> Vec<u8> {

    // Change to python directory
    let file_path = format!("{}{}", PROJECT_ROOT, RNN_PATH);
    set_relative_path(&file_path);

    // Call predict.py
    let output = Command::new("python")
                    .args(&[PREDICTOR, word])
                    .output()
                    .expect("Failed to execute process.");

    println!("{}", str::from_utf8(&output.stdout).unwrap());

    output.stdout.clone()
}

fn get_slices(phoneme_string : &Vec<u8>) -> Vec<&[u8]> {

    let mut phoneme_slices : Vec<&[u8]> = Vec::new(); 
    let mut left_bound : i32 = -1;
    for x in 0.. phoneme_string.len() {

        if phoneme_string[x] >= 'A' as u8 && phoneme_string[x] <= 'Z' as u8 {
            if left_bound == -1 {
                left_bound = x as i32;
            }
        } else if left_bound != -1 {
            phoneme_slices.push(&phoneme_string[(left_bound as usize).. (x as usize)]); 
            left_bound = -1;
        }

    }

    phoneme_slices
}

fn main() {
    
    let args : Vec<String> = env::args().collect(); 

    let phoneme_string = predict(args[1].as_str());
    let phoneme_slices = get_slices(&phoneme_string);
    
    let mut wave_files : Vec<Wave> = Vec::new();

    // Change path to audio file directory and generate slices
    let file_path = format!("{}{}", PROJECT_ROOT, AUDIO_FILE_PATH);
    set_relative_path(&file_path);
    
    for phoneme in phoneme_slices {
        let file_name = format!("{}.wav", str::from_utf8(phoneme).unwrap());
        wave_files.push(Wave::read_wav(&file_name));
    }
   
    // Change path to project root to spit out audio file
    set_relative_path(&PROJECT_ROOT);
    let mut merged_file = wave_files.remove(0);
    for x in 0.. wave_files.len() {
        merged_file.append(&mut wave_files[x]);
    }
    merged_file.write_to_file("merged.wav");

}

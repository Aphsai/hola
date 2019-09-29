
extern crate cpython;
mod wave;

use crate::wave::*;
use std::process::Command;
use std::str;
use std::env;

const RNN_PATH : &str = "rnn";
const PREDICTOR : &str = "predict.py";

fn predict() {
    // Change to python directory
    let mut current_directory = env::current_dir().unwrap();
    current_directory.push(RNN_PATH);
    env::set_current_dir(current_directory.as_path()).unwrap();
    let output = Command::new("python")
                    .args(&[PREDICTOR, "sarah"])
                    .output()
                    .expect("Failed to execute process.");
    println!("{}", str::from_utf8(&(output.stdout)).unwrap());
}

fn main() {
    
    let mut sample_one_data : Wave = Wave::read_wav("sample_1.wav");    
    let mut sample_two_data : Wave = Wave::read_wav("sample_2.wav");
    sample_one_data.append(&mut sample_two_data);
    sample_one_data.write_to_file("merged.wav");
    predict()

}

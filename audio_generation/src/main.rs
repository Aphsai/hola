mod wave;

use crate::wave::*;


fn main() {
    let mut sample_one_data : Wave = Wave::read_wav("sample_1.wav");    
    let mut sample_two_data : Wave = Wave::read_wav("sample_2.wav");
    sample_one_data.append(&mut sample_two_data);
    sample_one_data.write_to_file("merged.wav");
}

use std::fs::File;
use std::io::prelude::*;

pub fn read_adc(path: &str) -> u64 {
    let mut adc_file = File::open(path).expect("Unable to open ADC file");
    let mut adc_value = String::new();
    
    adc_file.read_to_string(&mut adc_value).expect("Unable to read ADC file");
    adc_value
        .trim()
        .parse::<u64>()
        .unwrap()
}
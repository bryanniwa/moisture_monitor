use std::fs;

pub fn read_adc(path: &str) -> u64 {
    let adc_value = fs::read_to_string(path).expect("Unable to read ADC file");
    adc_value
        .trim()
        .parse::<u64>()
        .unwrap()
}
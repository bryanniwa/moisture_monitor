mod adc;

use std::{thread, time};

const ADC_PATH: &str = "/sys/bus/iio/devices/iio:device0/in_voltage0_raw";

fn main() {
    loop {
        let adc_val = adc::read_adc(ADC_PATH);
        println!("ADC: {}", adc_val);
        thread::sleep(time::Duration::from_secs(1))
    }
}
